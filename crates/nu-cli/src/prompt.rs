use nu_utils::enable_vt_processing;
use reedline::{
    DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus,
    PromptViMode,
};
use std::borrow::Cow;
#[cfg(windows)]
use std::sync::Arc;

/// Nushell prompt definition
#[derive(Default, Clone)]
pub struct NushellPrompt {
    left_prompt: Option<String>,
    right_prompt: Option<String>,
    prompt_indicator: Option<String>,
    vi_insert_prompt_indicator: Option<String>,
    vi_normal_prompt_indicator: Option<String>,
    multiline_indicator: Option<String>,
    render_right_prompt_on_last_line: bool,
    right_prompt_fn: Option<Arc<dyn Fn() -> String + Send + Sync>>,
}

impl NushellPrompt {
    pub fn new() -> NushellPrompt {
        NushellPrompt {
            left_prompt: None,
            right_prompt: None,
            prompt_indicator: None,
            vi_insert_prompt_indicator: None,
            vi_normal_prompt_indicator: None,
            multiline_indicator: None,
            render_right_prompt_on_last_line: false,
            right_prompt_fn: None,
        }
    }

    pub fn update_prompt_left(&mut self, left_prompt_string: Option<String>) {
        self.left_prompt = left_prompt_string;
    }

    pub fn update_prompt_right(
        &mut self,
        right_prompt_string: Option<String>,
        render_right_prompt_on_last_line: bool,
    ) {
        self.right_prompt = right_prompt_string;
        self.render_right_prompt_on_last_line = render_right_prompt_on_last_line;
    }

    pub fn set_right_prompt_fn(&mut self, f: Arc<dyn Fn() -> String + Send + Sync>) {
        self.right_prompt_fn = Some(f);
    }

    pub fn update_prompt_indicator(&mut self, prompt_indicator_string: Option<String>) {
        self.prompt_indicator = prompt_indicator_string;
    }

    pub fn update_prompt_vi_insert(&mut self, prompt_vi_insert_string: Option<String>) {
        self.vi_insert_prompt_indicator = prompt_vi_insert_string;
    }

    pub fn update_prompt_vi_normal(&mut self, prompt_vi_normal_string: Option<String>) {
        self.vi_normal_prompt_indicator = prompt_vi_normal_string;
    }

    pub fn update_prompt_multiline(&mut self, prompt_multiline_indicator_string: Option<String>) {
        self.multiline_indicator = prompt_multiline_indicator_string;
    }

    pub fn update_all_prompt_strings(
        &mut self,
        left_prompt_string: Option<String>,
        right_prompt_string: Option<String>,
        prompt_indicator_string: Option<String>,
        prompt_multiline_indicator_string: Option<String>,
        prompt_vi: (Option<String>, Option<String>),
        render_right_prompt_on_last_line: bool,
    ) {
        let (prompt_vi_insert_string, prompt_vi_normal_string) = prompt_vi;

        self.left_prompt = left_prompt_string;
        self.right_prompt = right_prompt_string;
        self.prompt_indicator = prompt_indicator_string;
        self.multiline_indicator = prompt_multiline_indicator_string;
        self.vi_insert_prompt_indicator = prompt_vi_insert_string;
        self.vi_normal_prompt_indicator = prompt_vi_normal_string;
        self.render_right_prompt_on_last_line = render_right_prompt_on_last_line;
        self.right_prompt_fn = None;
    }

    fn default_wrapped_custom_string(&self, str: String) -> String {
        format!("({str})")
    }
}

impl Prompt for NushellPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        #[cfg(windows)]
        {
            let _ = enable_vt_processing();
        }

        if let Some(prompt_string) = &self.left_prompt {
            prompt_string.replace('\n', "\r\n").into()
        } else {
            let default = DefaultPrompt::default();
            default
                .render_prompt_left()
                .to_string()
                .replace('\n', "\r\n")
                .into()
        }
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        // If a lazy closure is set, call it now (evaluated at render time = when Enter is pressed).
        if let Some(ref f) = self.right_prompt_fn {
            return f().replace('\n', "\r\n").into();
        }
        if let Some(prompt_string) = &self.right_prompt {
            prompt_string.replace('\n', "\r\n").into()
        } else {
            let default = DefaultPrompt::default();
            default
                .render_prompt_right()
                .to_string()
                .replace('\n', "\r\n")
                .into()
        }
    }

    fn render_prompt_indicator(&self, edit_mode: PromptEditMode) -> Cow<'_, str> {
        let indicator: &str = match edit_mode {
            PromptEditMode::Default => self.prompt_indicator.as_deref().unwrap_or("> "),
            PromptEditMode::Emacs => self.prompt_indicator.as_deref().unwrap_or("> "),
            PromptEditMode::Vi(vi_mode) => match vi_mode {
                PromptViMode::Normal => self.vi_normal_prompt_indicator.as_deref().unwrap_or("> "),
                PromptViMode::Insert => self.vi_insert_prompt_indicator.as_deref().unwrap_or(": "),
            },
            PromptEditMode::Custom(str) => &self.default_wrapped_custom_string(str),
        };

        indicator.to_string().into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        let indicator = match &self.multiline_indicator {
            Some(indicator) => indicator.as_str(),
            None => "::: ",
        };

        indicator.to_string().into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {})",
            prefix, history_search.term
        ))
    }

    fn right_prompt_on_last_line(&self) -> bool {
        self.render_right_prompt_on_last_line
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    #[test]
    fn default_prompt_does_not_embed_osc_markers() {
        let prompt = NushellPrompt::new();
        let rendered = prompt.render_prompt_left().to_string();

        assert!(!rendered.contains("\x1b]133;"));
        assert!(!rendered.contains("\x1b]633;"));
    }

    #[test]
    fn right_prompt_fn_overrides_static_right_prompt() {
        let mut prompt = NushellPrompt::new();

        // Set a static right prompt string.
        prompt.update_prompt_right(Some("static-value".to_string()), false);

        // Set a lazy closure that returns a different value.
        let lazy_fn: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(|| "lazy-value".to_string());
        prompt.set_right_prompt_fn(lazy_fn);

        // The closure must win over the static string.
        assert_eq!(
            prompt.render_prompt_right().as_ref(),
            "lazy-value",
            "right_prompt_fn must take priority over right_prompt"
        );
    }

    #[test]
    fn right_prompt_fn_is_not_called_until_render() {
        let was_called = Arc::new(AtomicBool::new(false));

        let was_called_clone = Arc::clone(&was_called);
        let lazy_fn: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(move || {
            was_called_clone.store(true, Ordering::SeqCst);
            "result".to_string()
        });

        let mut prompt = NushellPrompt::new();

        // Setting the closure must NOT call it.
        prompt.set_right_prompt_fn(lazy_fn);
        assert!(
            !was_called.load(Ordering::SeqCst),
            "closure must not be called during set_right_prompt_fn"
        );

        // Rendering must call it.
        let _ = prompt.render_prompt_right();
        assert!(
            was_called.load(Ordering::SeqCst),
            "closure must be called during render_prompt_right"
        );
    }

    #[test]
    fn render_prompt_right_falls_back_to_static_when_no_fn() {
        let mut prompt = NushellPrompt::new();

        prompt.update_prompt_right(Some("hello-static".to_string()), false);

        // No right_prompt_fn is set; must return the static string.
        assert_eq!(
            prompt.render_prompt_right().as_ref(),
            "hello-static",
            "render_prompt_right must return the static string when no fn is set"
        );
    }

    #[test]
    fn update_all_prompt_strings_clears_right_prompt_fn() {
        let mut prompt = NushellPrompt::new();

        // Set a lazy closure first.
        let lazy_fn: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(|| "lazy-stale".to_string());
        prompt.set_right_prompt_fn(lazy_fn);

        // Confirm the closure is active.
        assert_eq!(prompt.render_prompt_right().as_ref(), "lazy-stale");

        // Now call update_all_prompt_strings, which must clear right_prompt_fn.
        prompt.update_all_prompt_strings(
            None,
            Some("new-static".to_string()),
            None,
            None,
            (None, None),
            false,
        );

        // The closure must be gone; the new static value must be returned.
        assert_eq!(
            prompt.render_prompt_right().as_ref(),
            "new-static",
            "update_all_prompt_strings must clear right_prompt_fn"
        );
    }
}
