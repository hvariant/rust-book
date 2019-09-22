pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            review_count: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    review_count: u32,
}

pub enum ApproveResult {
    Approved(Post),
    StillPending(PendingReviewPost),
}

impl PendingReviewPost {
    pub fn approve(self) -> ApproveResult {
        use ApproveResult::*;

        if self.review_count > 0 {
            Approved(Post {
                content: self.content,
            })
        } else {
            StillPending(PendingReviewPost {
                content: self.content,
                review_count: 1,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
