#![allow(dead_code)]

trait State : std::fmt::Debug {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn state_name(&self) -> String {
        format!("{:?}", self).to_string()
    }
    fn pass_through<'a >(&self, _text: &'a str) -> &'a str {
        ""
    }

    /// TODO: Complete implementation
    fn update_post<'a>(&self, _post: &'a mut Post, _text: &'a str) {
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft::new())),
            content: String::new(),
        }
    }

    pub fn state(&self) -> String {
        match self.state.as_ref() {
            Some(s) => s.state_name(),
            None => "None".to_owned(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.as_ref() {
            let text = s.pass_through(text);
            self.content.push_str(text);
        }
    }

    /// TODO: Complete implementation
    pub fn update_post(&mut self, _text: &str) {
        if let Some(_s) = self.state.as_ref() {
            // s.update_post(self, text);
            println!("TODO: Complete implementation: s.update_post(self, text);")
        }
    }

    pub fn content(&self) -> &str {
        // Expanded version
        let s_ref = self.state.as_ref();
        let s_box = s_ref.unwrap();
        let s_cnt = s_box.content(self);
        &s_cnt

        // Compact version
        // self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }
}

#[derive(Debug)]
struct Draft {}

impl Draft {
    pub fn new() -> Self {
        Draft {}
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn pass_through<'a >(&self, _text: &'a str) -> &'a str {
        _text
    }

    /// TODO: Complete implementation
    fn update_post<'a>(&self, _post: &'a mut Post, text: &'a str) {
        _post.content.push_str(text);
    }
}

#[derive(Debug)]
struct PendingReview {}

impl PendingReview {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReviewFinal {})
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft::new())
    }
}

#[derive(Debug)]
struct PendingReviewFinal {}

impl State for PendingReviewFinal {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft::new())
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
