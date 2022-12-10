// Take a look at the license at the top of the repository in the LICENSE file.

use std::iter::FusedIterator;

use crate::{prelude::*, FileEnumerator, FileInfo};

impl Iterator for FileEnumerator {
    type Item = Result<FileInfo, glib::Error>;

    fn next(&mut self) -> Option<Result<FileInfo, glib::Error>> {
        match self.next_file(crate::Cancellable::NONE) {
            Err(err) => Some(Err(err)),
            Ok(file_info) => file_info.map(Ok),
        }
    }
}

impl FusedIterator for FileEnumerator {}
