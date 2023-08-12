#[cfg(test)]
mod tests {
    use semver::{Version, VersionReq};

    use crate::structs::{
        forgemod::ForgeMod,
        v1::{IncludeDataBuilder, ManifestBuilder, ModBuilder},
    };

    #[test]
    fn test_make_mod() {
        let mut _tmod = ModBuilder::new_mod_raw(
            ManifestBuilder::new_mod(
                "pp".to_string(),
                Version::new(0, 1, 2),
                VersionReq::parse("=1.23.4").unwrap(),
                "./test.dll".into(),
            )
            .build(),
            vec![0xFF, 0xFF],
        );
        _tmod.includes(IncludeDataBuilder::new().add_raw("./Plugins".to_string(), vec![0xFF, 0xFF]).clone().build());

        let bin = _tmod.clone().build().pack().unwrap();
        let tmod2 = ForgeMod::from_bytes(&*bin).unwrap();
        assert_eq!(_tmod.build(), tmod2)
    }

    #[test]
    fn test_make_lib() {
        let mut _tmod = ModBuilder::new_lib_raw(
            ManifestBuilder::new_lib(
                "pp".to_string(),
                Version::new(0, 1, 2),
                VersionReq::parse("=1.23.4").unwrap(),
                "./test.dll".into(),
            )
            .build(),
            vec![0xFF, 0xFF],
        );
        _tmod.includes(IncludeDataBuilder::new().add_raw("./Plugins".to_string(), vec![0xFF, 0xFF]).clone().build());

        let bin = _tmod.clone().build().pack().unwrap();
        let tmod2 = ForgeMod::from_bytes(&*bin).unwrap();
        assert_eq!(_tmod.build(), tmod2)
    }

    #[test]
    fn test_make_module_parent() {
        let mut _tmod = ModBuilder::new_module_parent(
            ManifestBuilder::new_module_parent(
                "pp".to_string(),
                Version::new(0, 1, 2),
                VersionReq::parse("=1.23.4").unwrap(),
            )
            .build(),
        );

        let bin = _tmod.clone().build().pack().unwrap();
        let tmod2 = ForgeMod::from_bytes(&*bin).unwrap();
        assert_eq!(_tmod.build(), tmod2)
    }

    #[test]
    fn test_make_module () {
        let mut _tmod = ModBuilder::new_module_raw(
            ManifestBuilder::new_module(
                "pp".to_string(),
                "./test.dll".into(),
            )
            .build(),
            vec![0xFF, 0xFF],
        );

        let bin = _tmod.clone().build().pack().unwrap();
        let tmod2 = ForgeMod::from_bytes(&*bin).unwrap();
        assert_eq!(_tmod.build(), tmod2)
    }
}