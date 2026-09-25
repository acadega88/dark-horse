use std::sync::mpsc::Sender;

use block2::RcBlock;
use objc2::MainThreadMarker;
use objc2_foundation::{NSError, NSString};
use objc2_web_kit::{WKContentRuleList, WKContentRuleListStore, WKUserContentController};

const RULE_LIST_ID: &str = "dark-horse-v0.0.9-test-rule";
const TEST_RULES: &str = r#"[
  {
    "trigger": {
      "url-filter": "^http://127\\.0\\.0\\.1:8765/blocked-resource$"
    },
    "action": { "type": "block" }
  },
  {
    "trigger": {
      "url-filter": "^http://localhost:8765/blocked-resource$"
    },
    "action": { "type": "block" }
  }
]"#;

fn rule_store() -> Option<objc2::rc::Retained<WKContentRuleListStore>> {
    let main_thread = MainThreadMarker::new()?;
    // SAFETY: WKContentRuleListStore.defaultStore is called on the app's main thread.
    unsafe { WKContentRuleListStore::defaultStore(main_thread) }
}

pub(super) fn compile_test_rule(sender: Sender<Result<(), String>>) {
    let Some(store) = rule_store() else {
        let _ = sender.send(Err(
            "Could not access WebKit's content rule store.".to_owned()
        ));
        return;
    };

    let identifier = NSString::from_str(RULE_LIST_ID);
    let rules = NSString::from_str(TEST_RULES);
    let callback_store = store.clone();
    let callback = RcBlock::new(
        move |rule_list: *mut WKContentRuleList, _error: *mut NSError| {
            let result = if rule_list.is_null() {
                Err("WebKit could not compile the test request filter.".to_owned())
            } else {
                Ok(())
            };
            let _ = sender.send(result);
            let _ = &callback_store;
        },
    );

    // SAFETY: The rule list JSON and identifier are valid NSString values, and WebKit retains
    // the completion block while it compiles the rule list.
    unsafe {
        store.compileContentRuleListForIdentifier_encodedContentRuleList_completionHandler(
            Some(&identifier),
            Some(&rules),
            Some(&callback),
        );
    }
}

pub(super) fn attach_test_rule(
    manager: objc2::rc::Retained<WKUserContentController>,
    tab_id: u64,
    sender: Sender<(u64, Result<(), String>)>,
) {
    let Some(store) = rule_store() else {
        let _ = sender.send((
            tab_id,
            Err("Could not access WebKit's content rule store.".to_owned()),
        ));
        return;
    };

    let identifier = NSString::from_str(RULE_LIST_ID);
    let callback_store = store.clone();
    let callback = RcBlock::new(
        move |rule_list: *mut WKContentRuleList, _error: *mut NSError| {
            let result = if rule_list.is_null() {
                Err("WebKit could not load the compiled request filter.".to_owned())
            } else {
                // SAFETY: The callback supplies a valid rule-list object for this invocation. The
                // controller belongs to the same WKWebView and is retained by this completion block.
                unsafe { manager.addContentRuleList(&*rule_list) };
                Ok(())
            };
            let _ = sender.send((tab_id, result));
            let _ = &callback_store;
        },
    );

    // SAFETY: The identifier is valid, and WebKit retains the completion block until lookup ends.
    unsafe {
        store.lookUpContentRuleListForIdentifier_completionHandler(
            Some(&identifier),
            Some(&callback),
        );
    }
}
