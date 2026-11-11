use std::sync::mpsc::Sender;

use adblock::content_blocking::CbRule;
use adblock::lists::{FilterSet, ParseOptions, RuleTypes};
use block2::RcBlock;
use objc2::MainThreadMarker;
use objc2_foundation::{NSError, NSString};
use objc2_web_kit::{WKContentRuleList, WKContentRuleListStore, WKUserContentController};

const RULE_LIST_ID: &str = "dark-horse-v0.0.9-easylist-202611110754";
const EASYLIST: &str = include_str!("../../assets/easylist-202611110754.txt");
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
    },
    {
        "trigger": {
            "url-filter": "^http://127\\.0\\.0\\.1:8765/third-party-blocked-resource$",
            "load-type": ["third-party"]
        },
        "action": { "type": "block" }
    },
    {
        "trigger": {
            "url-filter": "^http://localhost:8765/third-party-blocked-resource$",
            "load-type": ["third-party"]
        },
        "action": { "type": "block" }
  }
]"#;

fn encoded_rules() -> Result<String, String> {
    let options = ParseOptions {
        rule_types: RuleTypes::NetworkOnly,
        ..ParseOptions::default()
    };
    let mut filter_set = FilterSet::new(true);
    filter_set.add_filter_list(EASYLIST.to_owned(), options);
    let (mut rules, _) = filter_set
        .into_content_blocking()
        .map_err(|_| "Could not convert EasyList rules to WebKit format.".to_owned())?;
    let test_rules: Vec<CbRule> = serde_json::from_str(TEST_RULES)
        .map_err(|error| format!("Could not add local request-filter tests: {error}"))?;
    rules.extend(test_rules);
    serde_json::to_string(&rules)
        .map_err(|error| format!("Could not encode WebKit request-filter rules: {error}"))
}

fn rule_store() -> Option<objc2::rc::Retained<WKContentRuleListStore>> {
    let main_thread = MainThreadMarker::new()?;
    // SAFETY: WKContentRuleListStore.defaultStore is called on the app's main thread.
    unsafe { WKContentRuleListStore::defaultStore(main_thread) }
}

pub(super) fn compile_filter_rules(sender: Sender<Result<(), String>>) {
    let rules = match encoded_rules() {
        Ok(rules) => rules,
        Err(error) => {
            let _ = sender.send(Err(error));
            return;
        }
    };
    let Some(store) = rule_store() else {
        let _ = sender.send(Err(
            "Could not access WebKit's content rule store.".to_owned()
        ));
        return;
    };

    let identifier = NSString::from_str(RULE_LIST_ID);
    let rules = NSString::from_str(&rules);
    let callback_store = store.clone();
    let callback = RcBlock::new(
        move |rule_list: *mut WKContentRuleList, _error: *mut NSError| {
            let result = if rule_list.is_null() {
                Err("WebKit could not compile the request filter.".to_owned())
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

pub(super) fn attach_filter_rules(
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

#[cfg(test)]
mod tests {
    use super::{CbRule, encoded_rules};

    #[test]
    fn easylist_converts_to_webkit_rules_and_keeps_local_fixture_rules() {
        let encoded_rules = encoded_rules().expect("EasyList should convert to WebKit rules");
        let rules: Vec<CbRule> =
            serde_json::from_str(&encoded_rules).expect("converted rules should be valid JSON");

        assert!(rules.len() > 48_000);
        assert!(rules.iter().any(|rule| {
            rule.trigger.url_filter == "^http://127\\.0\\.0\\.1:8765/blocked-resource$"
        }));
        assert!(rules.iter().any(|rule| {
            rule.trigger.url_filter == "^http://127\\.0\\.0\\.1:8765/third-party-blocked-resource$"
                && !rule.trigger.load_type.is_empty()
        }));
    }
}
