// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(test)]
mod tests {
    use crate::translate::*;
    use crate::DateTime;
    use crate::ToValue;

    #[test]
    fn test_value() {
        let dt1 = DateTime::now_utc().unwrap();
        let v = dt1.to_value();
        let dt2 = v.get::<&DateTime>().unwrap();

        assert_eq!(dt1.to_glib_none().0, dt2.to_glib_none().0);
    }
}
