mod catalog {
    pub mod genres {
        #[derive(Debug)]
        enum Genre {
            Rock,
            Pop,
            Jazz,
            Classical,
            HipHop,
        }

        pub fn all() -> Vec<Genre> {
            vec![
                Genre::Rock,
                Genre::Pop,
                Genre::Jazz,
                Genre::Classical,
                Genre::HipHop,
            ]
    }

    #[derive(Debug)]
    pub struct Song {
        pub title: String,
        pub artist: String,
        pub genre: Genre,
    }

    impl Song {
        pub fn new(title: &str, artist: &str, genre: Genre) -> Song {
            Song {
                title: title.to_string(),
                artist: artist.to_string(),
                genre,
            }
        }
    }


}