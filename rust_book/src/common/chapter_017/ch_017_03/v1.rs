#![allow(dead_code)]

// Initial implementation of the state pattern
// 
// Allow users to add text content only when a post is in the Draft state - based on explicit type check

use std::any::{Any};

/// NOTE: 
/// For the `trait State : std::fmt::Debug {`
/// `format!("{:?}", self).to_string()` below converts to string a type of an object that trait object points to
trait State : std::fmt::Debug {
// trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    #[allow(unused_variables)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn state_name(&self) -> String {
        format!("{:?}", self)
    }
    fn as_any(&self) -> &dyn Any;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
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

        // Trait type check v1 - Expanded with verbosity
        match self.state.as_ref() {
            Some(state) => {
                match state.as_any().downcast_ref::<Draft>() {
                    Some(_) => {
                        self.content.push_str(text);
                    },
                    _ => {
                        println!();
                        println!("    Post::add_text skipped,");
                        println!("    State is not a Draft,");
                        println!("    Skipped text:");
                        println!("        {:?}", text);
                        println!();
                    },
                }
            },
            None => {
                println!();
                println!("    Post::add_text skipped,");
                println!("    State is None,");
                println!("    Skipped text:");
                println!("        {:?}", text);
                println!();
            },
        }

        // // Trait type check v1 - Compact without verbosity
        // match self.state.as_ref() {
        //     Some(state) => match state.as_any().downcast_ref::<Draft>() {
        //         Some(_) => self.content.push_str(text),
        //         _ => (),
        //     },
        //     None => (),
        // }

        // // Trait type check v2
        // let d = self.state.as_deref();

        // if let Some(d) = d {
        //     match d.as_any().downcast_ref::<Draft>() {
        //         Some(_) => {
        //             self.content.push_str(text);
        //         },
        //         _ => {
        //             println!();
        //             println!("    Post::add_text skipped,");
        //             println!("    State is not a Draft,");
        //             println!("    Skipped text:");
        //             println!("        :{:?}", text);
        //             println!();
        //         },
        //     };
        // }
    }

    pub fn content(&self) -> &str {
        // Expanded version
        let s_ref = self.state.as_ref();
        let s_box = s_ref.unwrap();
        let s_cnt = s_box.content(self);
        s_cnt

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

    fn as_any(&self) -> &dyn Any {
        self
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
        Box::new(Draft {})
    }

    fn as_any(&self) -> &dyn Any {
        self
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
        Box::new(Draft {})
    }

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
