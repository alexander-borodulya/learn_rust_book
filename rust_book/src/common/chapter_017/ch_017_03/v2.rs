#![allow(dead_code)]

/// Encoding States and Behavior as Types

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn new_post_with_string(content: String) -> Post {
        Post { content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn new_post_with_string(content: String) -> DraftPost {
        DraftPost { content }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost::new(self.content)
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn new(content: String) -> PendingReviewPost {
        PendingReviewPost { content }
    }
    
    pub fn approve(self) -> PendingFinalReviewPost {
        PendingFinalReviewPost::new(self.content)
    }
    
    pub fn reject(self) -> DraftPost {
        DraftPost::new_post_with_string(self.content)
    }
}

pub struct PendingFinalReviewPost {
    content: String,
}

impl PendingFinalReviewPost {
    pub fn new(content: String) -> PendingFinalReviewPost {
        PendingFinalReviewPost { content }
    }
    
    pub fn approve(self) -> Post {
        Post::new_post_with_string(self.content)
    }
    
    pub fn reject(self) -> DraftPost {
        DraftPost::new_post_with_string(self.content)
    }
}
