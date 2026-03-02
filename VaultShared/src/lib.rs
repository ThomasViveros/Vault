pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//Index
/*
 * What is the index? The index is the manager of version control. It contains information about
 * each version and what files belong in that version. The index isnt responsible for compressing
 * and storing files, simply tracking them.
 *
 * But what file type should it be? Perhaps json? Should we make a custom file type?
 * Well, the good thing about the index and versions, is that you dont need to go back and edit
 * versions once there cemented. So we could tightly pack versions. We would need a way to
 * determine*/
