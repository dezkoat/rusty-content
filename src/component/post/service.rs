pub struct PostService {
    
}

impl PostService {

    pub fn new() -> PostService {
        PostService {}
    }

    pub fn post_read_all(&self) -> String {
        String::from("readall")
    }

    pub fn post_read(&self) -> String {
        String::from("read")
    }

    pub fn post_create(&self) -> String {
        String::from("Test")
    }

    pub fn post_update(&self) -> String {
        String::from("Test")
    }

    pub fn post_delete(&self) -> String {
        String::from("Test")
    }
}