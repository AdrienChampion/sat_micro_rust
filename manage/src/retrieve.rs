//! Retrieves the benchmarks.

prelude!();

use std::sync::RwLock;

use clap::SubCommand;

#[derive(Debug)]
pub struct Retrieve {
    errs: RwLock<Vec<err::Error>>,
    tgt: PathBuf,
}
impl Retrieve {
    pub const SUBCOMMAND_NAME: &'static str = "retrieve";
    const TGT_DIR_ARG: &'static str = "RETRIEVE_TGT_DIR";

    pub fn subcommand() -> App {
        SubCommand::with_name(Self::SUBCOMMAND_NAME)
            .about("retrieves the benchmarks from SAT-COMP 2020")
            .arg(
                clap::Arg::with_name(Self::TGT_DIR_ARG)
                    .help("Target directory (will be created if necessary)")
                    .default_value("./rsc"),
            )
    }

    /// Constructor.
    pub fn new(matches: &Matches) -> Res<Option<Self>> {
        let matches = if let Some(m) = matches.subcommand_matches(Self::SUBCOMMAND_NAME) {
            m
        } else {
            return Ok(None);
        };
        let tgt: PathBuf = matches
            .value_of(Self::TGT_DIR_ARG)
            .expect("unwrap of argument with default value cannot fail")
            .into();

        if tgt.exists() && !tgt.is_dir() {
            bail!(
                "target directory for benchmarks `{}` is not a directory",
                tgt.display()
            )
        }

        Ok(Some(Self {
            errs: RwLock::new(vec![]),
            tgt,
        }))
    }

    async fn dl(uri: &str) -> Res<()> {
        let response = reqwest::get(uri).await?;

        println!("headers: {:?}", response.headers());

        Ok(())
    }

    /// Retrieves all benchmarks to `rsc/sat_comp`.
    pub fn run(self) -> Result<(), Vec<err::Error>> {
        let mut uris = URIS.lines();
        let first = uris.next().unwrap();
        match futures::executor::block_on(Self::dl(first)) {
            Ok(()) => (),
            Err(e) => {
                let mut errs = self
                    .errs
                    .write()
                    .expect("[fatal] lock on error list was poisoned");
                errs.push(e);
            }
        }

        let errs = self
            .errs
            .into_inner()
            .expect("[fatal] lock on error list was poisoned");

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

const URIS: &str = "\
https://gbd.iti.kit.edu/file/0151bedac526ee195bc52e4134cd80e7
https://gbd.iti.kit.edu/file/0241f35c5752768d2b0580533d143a14
https://gbd.iti.kit.edu/file/02627689047d06fbb642eef14768d751
https://gbd.iti.kit.edu/file/02c6fe8483e4f4474b7ac9731772535d
https://gbd.iti.kit.edu/file/02f2343e32f9070f149708d77556b4aa
https://gbd.iti.kit.edu/file/031757c0a6797eddf3e84bcefbbdb753
https://gbd.iti.kit.edu/file/03fb9af7b390fe9e0739150ca3410cf0
https://gbd.iti.kit.edu/file/04157f716c1e9606c6a530657bf8f957
https://gbd.iti.kit.edu/file/04557a22f623dcbae86173fced057c99
https://gbd.iti.kit.edu/file/08abbb56b2345b2da7af8866a4e8ad44
https://gbd.iti.kit.edu/file/08e151e72fe10402a49463171aa557e8
https://gbd.iti.kit.edu/file/0928111a3d5d5ce05dffb83cb5982eba
https://gbd.iti.kit.edu/file/09f4f1e3047315b5ef2690fffb7ccf7a
https://gbd.iti.kit.edu/file/0ae917c2e0c1d39cbe411c29b2f5ff08
https://gbd.iti.kit.edu/file/0b1041a1e55af6f3d2c63462a7400bd2
https://gbd.iti.kit.edu/file/0b348eb6b61e4f0026437fda402c9b62
https://gbd.iti.kit.edu/file/0d7f17f04ed073a8ea39e72783186cdb
https://gbd.iti.kit.edu/file/0f46a676eab0d62b0b59356cb39e22ae
https://gbd.iti.kit.edu/file/0f8d82f07dc5353a4f6e0c699abb8c52
https://gbd.iti.kit.edu/file/0ff0fc8ef80d32742d38231869d5d244
https://gbd.iti.kit.edu/file/100193c6b2ba32b40859f2bb99ec269c
https://gbd.iti.kit.edu/file/102ddb414ef5a0f8cf06f168df8f271a
https://gbd.iti.kit.edu/file/11f7856b53db9263e48e81a61b0573fd
https://gbd.iti.kit.edu/file/12bb20e3ad925897977f85af71db7c94
https://gbd.iti.kit.edu/file/12d79233413fe38d99a604487d2c3515
https://gbd.iti.kit.edu/file/1446548d272c0b28fb40225c50d743a5
https://gbd.iti.kit.edu/file/14970e78625100605d22c9916dd0a476
https://gbd.iti.kit.edu/file/15300be1a87777f0110722557a86bf7a
https://gbd.iti.kit.edu/file/16b9e9e007133b818ec1f0868613d534
https://gbd.iti.kit.edu/file/16d83e40878ebec355597e0edb0e0c72
https://gbd.iti.kit.edu/file/180e6d8b61fe6bab26126b78591e324e
https://gbd.iti.kit.edu/file/1aa8a801f9c01f07c52e19d7a005233d
https://gbd.iti.kit.edu/file/1b14628a6605fc607c6d99d3d783251a
https://gbd.iti.kit.edu/file/1c1d04b20da61c0bc2328cdca8b8aa3a
https://gbd.iti.kit.edu/file/1e8af331b822f6b7bc1754be9d64499f
https://gbd.iti.kit.edu/file/1f1ab0fd7da480fb9736da2b3e985fb8
https://gbd.iti.kit.edu/file/21a16f532f37b506ba9b59952a40a2b6
https://gbd.iti.kit.edu/file/21fc09c6badfc60aaa85cae664023cc6
https://gbd.iti.kit.edu/file/23ddc17bdbd97b96fcc0393f51bcfe13
https://gbd.iti.kit.edu/file/24ea04bb401629158b0972463c61eed4
https://gbd.iti.kit.edu/file/276fe9730ff09cafd8c59f44c9c61b4d
https://gbd.iti.kit.edu/file/29be431e9dee1550957c8483be4532e4
https://gbd.iti.kit.edu/file/2a15a30186afdad41a49c5c5366d01be
https://gbd.iti.kit.edu/file/2a3365659589da4f9812e5d96d3935b0
https://gbd.iti.kit.edu/file/2a6bfb04f247a0790162269a5c8ec071
https://gbd.iti.kit.edu/file/2b5f3caa6d84857f4919513df087bbdf
https://gbd.iti.kit.edu/file/2c216cf84cc3a61aaa6d7e6afaeaf5af
https://gbd.iti.kit.edu/file/2c5452e8b90e57f4e0900438ff749f20
https://gbd.iti.kit.edu/file/2d0c041c0fe72dc32527bfbf34f63e61
https://gbd.iti.kit.edu/file/2e29192bee7f468c3c6e1b7088829575
https://gbd.iti.kit.edu/file/2f998f06e0729ffa9ee5930dd8ba807c
https://gbd.iti.kit.edu/file/310c8feaba5b17320db0b224bfb9478b
https://gbd.iti.kit.edu/file/31746b8d1bf7628c843592a85a8d95ba
https://gbd.iti.kit.edu/file/32076b11a5d315b752dfdc106c861cb1
https://gbd.iti.kit.edu/file/3403ba1f14b27a23c95527c490058766
https://gbd.iti.kit.edu/file/34323d942ba720695022bb1251190a01
https://gbd.iti.kit.edu/file/343885733b441b8dbf2d90886f39c746
https://gbd.iti.kit.edu/file/366397b26e0b86f275f5f4b827bde95e
https://gbd.iti.kit.edu/file/38295b28f74d2a62e928c221428faf36
https://gbd.iti.kit.edu/file/3c0e26a50e17f2aee72383f5dfe81ee4
https://gbd.iti.kit.edu/file/3c1252e11adea78ca57afd0f9ed8f8fe
https://gbd.iti.kit.edu/file/3d0b118834cdf8e692a47ac623778b1d
https://gbd.iti.kit.edu/file/3d38ffe08887da6cbe9b17ce50c4b34c
https://gbd.iti.kit.edu/file/3e29b1449c9b29120ffac84634f6e5f5
https://gbd.iti.kit.edu/file/3e912cd2a499b8f956246b866e2f9f64
https://gbd.iti.kit.edu/file/3e9ce48792c9a6e1727393f09c28fad3
https://gbd.iti.kit.edu/file/3f266eb6ee4251b947c582f7c6a59d67
https://gbd.iti.kit.edu/file/3f4946b0d78b8b112b8e7737ae6db0f1
https://gbd.iti.kit.edu/file/3f9985018dd0c9655cdb7063258d0279
https://gbd.iti.kit.edu/file/3fd8869c1a5a7b8e4889a530e1ed2ea7
https://gbd.iti.kit.edu/file/417807e4c1f2829471bcd8981304b2d6
https://gbd.iti.kit.edu/file/43617e47d7f29ff77cc9917919cff6f5
https://gbd.iti.kit.edu/file/441ea208de1be6e32e97415c9da9241b
https://gbd.iti.kit.edu/file/45a09efb026036ff4b8d19024a7563a9
https://gbd.iti.kit.edu/file/47b7849bdc0eefee88c791a9ced970af
https://gbd.iti.kit.edu/file/4a00f482957850eed1b7ac2ec53d43ea
https://gbd.iti.kit.edu/file/4afd946cbaa6f41aa61dda47347dc973
https://gbd.iti.kit.edu/file/4e6ed02919858555623b578af7bb9ce5
https://gbd.iti.kit.edu/file/4ea29c4f64c7d368f2cb7d4319a8ce1e
https://gbd.iti.kit.edu/file/514f36038ad462677ca722a1d55cac13
https://gbd.iti.kit.edu/file/533017e0f42d8246c7ae140313a84161
https://gbd.iti.kit.edu/file/53463611103e9b4523eef1d9b5b0f105
https://gbd.iti.kit.edu/file/53a7ac1b3073205e8525b2379ab77554
https://gbd.iti.kit.edu/file/53ffd199202cc7280149f6dda10dc670
https://gbd.iti.kit.edu/file/552b8976b37b0d8b58bc8edad91c5695
https://gbd.iti.kit.edu/file/5690b9b0380aa9508699e56cae5918b5
https://gbd.iti.kit.edu/file/592078a0b65e96c18e866d4a96d81bab
https://gbd.iti.kit.edu/file/5a5fb82a3672ee898465aa8f1103147f
https://gbd.iti.kit.edu/file/5d357f1e4b5abc96c90d305f629edd5c
https://gbd.iti.kit.edu/file/5d3f7a547ff9818a05747963c1930f71
https://gbd.iti.kit.edu/file/5d65c8ec63b606c54ed75f0f3e0529e6
https://gbd.iti.kit.edu/file/5d7d18cfa2b71bbf56c11f2fdfbd6873
https://gbd.iti.kit.edu/file/5dea66a04fbaecf10dce69fa9c0e3665
https://gbd.iti.kit.edu/file/5fea03f266f60a417e85af49a5d769e5
https://gbd.iti.kit.edu/file/602a8ce027e4e0fb19f782c576516bb7
https://gbd.iti.kit.edu/file/60522b751c6ec8f1f02e0c95ae60b721
https://gbd.iti.kit.edu/file/6088fba41b1711d09d7ea147870434d0
https://gbd.iti.kit.edu/file/60e9b195b9e92db3cfe16916d53757c0
https://gbd.iti.kit.edu/file/615c30ee68ba7f9007a4511f7e8ab939
https://gbd.iti.kit.edu/file/6392cea0e1c5c95773c79cd143e2a934
https://gbd.iti.kit.edu/file/63acdb1e4c4437b22b7ee3b405c1d096
https://gbd.iti.kit.edu/file/651ba27349b792ee6abdf5ab8e7a14a0
https://gbd.iti.kit.edu/file/673c1a0da2eb39474b2f50569f49e303
https://gbd.iti.kit.edu/file/681b6efef89c7fbb930b76ea0ab917de
https://gbd.iti.kit.edu/file/6b10e4ddf39b316ffe87d148a4626ed4
https://gbd.iti.kit.edu/file/6b2496b2ba23a00a1c6f45f87faeba5f
https://gbd.iti.kit.edu/file/6b4f076f8ed55503b6a417dfe76dd35b
https://gbd.iti.kit.edu/file/6d04eefb847e8a70fdf3394c22cc794c
https://gbd.iti.kit.edu/file/6d094ef8ff8403c1338136bdc474b1c9
https://gbd.iti.kit.edu/file/6d9b2fef1a2d5d26c96d654d1d85d06d
https://gbd.iti.kit.edu/file/6eb1a684231ecfd5ba0617abfaa2c632
https://gbd.iti.kit.edu/file/6eb486d72b1bfd46e22d20e366e85597
https://gbd.iti.kit.edu/file/6f82682182394eb554a47a55f6c4453e
https://gbd.iti.kit.edu/file/70628bc527228ff61fc0ebfc3a6ff20f
https://gbd.iti.kit.edu/file/709d0994d0c356b339b051a8e62d998a
https://gbd.iti.kit.edu/file/70a8711118d4eaf15674ebc71bfb7c35
https://gbd.iti.kit.edu/file/70d67c4d9aeb1412df80f33a06121cb9
https://gbd.iti.kit.edu/file/72a1b82b2c1f17b0e5e0d4a44937d848
https://gbd.iti.kit.edu/file/74a7c2b5fbcf433b1bac43748c09e0ff
https://gbd.iti.kit.edu/file/760d32fe356e344f46a97fed6f51687b
https://gbd.iti.kit.edu/file/770180d477dbbf2d68c8812bb0a235e5
https://gbd.iti.kit.edu/file/77d8d18a001b2f7ae4208299ddcf1422
https://gbd.iti.kit.edu/file/792b33e1963e294682d2188f36554008
https://gbd.iti.kit.edu/file/796f0662f0c7b23a00dec660d6622806
https://gbd.iti.kit.edu/file/7a13ffe9546cc7dfabdedebb64700e8a
https://gbd.iti.kit.edu/file/7a5e40de64066f1886156ce2f6ad20fb
https://gbd.iti.kit.edu/file/7a955f7b50c9e274fdd477006a02d609
https://gbd.iti.kit.edu/file/7af73f846548141cc97b8a149a20bfbf
https://gbd.iti.kit.edu/file/7b430dc4761000c3404b608fd5b1123e
https://gbd.iti.kit.edu/file/7e2cceaad4188bcf9a915ebff0835157
https://gbd.iti.kit.edu/file/8117a2ac08e1acf52f660663efe2a5ca
https://gbd.iti.kit.edu/file/817ce8d840cd29ce88294362ec9a3a5a
https://gbd.iti.kit.edu/file/81c36c1f55a6ee8e180f53f140ce05a3
https://gbd.iti.kit.edu/file/81f247cf04746d7e0fb3ea77a760a0af
https://gbd.iti.kit.edu/file/8356e57e0e64b86e28bedb6aff099c96
https://gbd.iti.kit.edu/file/8483632ccd23319f4710174391d52c4f
https://gbd.iti.kit.edu/file/849878639fdefb334f90392a4cfc1548
https://gbd.iti.kit.edu/file/84ace37814debb04200a7cf10e31f896
https://gbd.iti.kit.edu/file/84ee8c52086baae8850e88cde56f4af9
https://gbd.iti.kit.edu/file/85071adde6e3118aef323066387a4aac
https://gbd.iti.kit.edu/file/8549a59214f5c35da1333eb1ee77df30
https://gbd.iti.kit.edu/file/856e8a73e5c92ccdb42ee5851e67d207
https://gbd.iti.kit.edu/file/85b3227365fe9bf60b08b456c550bc85
https://gbd.iti.kit.edu/file/85eef885042cc63551c8938d4f6ab02e
https://gbd.iti.kit.edu/file/885a46fd38d5cb4388406ee286babc5e
https://gbd.iti.kit.edu/file/88849daabda2e712f659c6edbc142229
https://gbd.iti.kit.edu/file/8971386b2aa05d4029deaead24340fff
https://gbd.iti.kit.edu/file/8a37400e66bf1cebf0145e95ee95da7f
https://gbd.iti.kit.edu/file/8abb66e873f3f161eaa26ea46c52987e
https://gbd.iti.kit.edu/file/8b463d3ac858ec91fb0056c0c17a0c75
https://gbd.iti.kit.edu/file/8b6635639b3d4c02c28f28410f286d85
https://gbd.iti.kit.edu/file/8bda0a67cfed5657a072d53e571d20ef
https://gbd.iti.kit.edu/file/8c70a8d5d537e2c49a58d3c3787ca602
https://gbd.iti.kit.edu/file/8d04ddb02298c335394dc78b177bb0e4
https://gbd.iti.kit.edu/file/8d212cd0912c67d6634a5ca8a5d9f7cb
https://gbd.iti.kit.edu/file/8e5e7c6eb2f50bf91a03da4958582ab4
https://gbd.iti.kit.edu/file/8e905dfa09f45f7f50099f70cc38714c
https://gbd.iti.kit.edu/file/9030e1888ba0aff5c7b0cf90adc47aaa
https://gbd.iti.kit.edu/file/90d6cb8b040b5ad926248e8a75c0fb07
https://gbd.iti.kit.edu/file/9276ce38c625b2d00de247f8588f1542
https://gbd.iti.kit.edu/file/92c17c00d862668de6e6c45d4eecdc75
https://gbd.iti.kit.edu/file/92f994c5f2fe934d4ba65f781db417d9
https://gbd.iti.kit.edu/file/931621d9964e9d0cd1c4001822015c92
https://gbd.iti.kit.edu/file/931a63080ec1b4a034b0bf3aa4edd6c8
https://gbd.iti.kit.edu/file/938be7a03ff1f4ac3b09faa40f7d12b4
https://gbd.iti.kit.edu/file/940aeb30818a6b0656c9906a32bcd7bc
https://gbd.iti.kit.edu/file/9412f2d96dc9130e236a3446eb70e0c0
https://gbd.iti.kit.edu/file/964f79e4878eb131f0c370bc5d2f2a0b
https://gbd.iti.kit.edu/file/98a6390be8264c04046d5245c449c061
https://gbd.iti.kit.edu/file/98c53d2e6e9cbf4a78a258adc529770a
https://gbd.iti.kit.edu/file/9a85720c365680d985924c786fabad81
https://gbd.iti.kit.edu/file/9b898146eb171d00db6a1c660e95011d
https://gbd.iti.kit.edu/file/9c43b8ab458d810c54ea7e5d3a967fc0
https://gbd.iti.kit.edu/file/9d7caee59e9d362d132f1cd785de8a0b
https://gbd.iti.kit.edu/file/9e7fcd9893a2abee4d76bc3b57d9ffa1
https://gbd.iti.kit.edu/file/9e867160f5dd14c2127a2e513f48b7d5
https://gbd.iti.kit.edu/file/a19a069b6c4b8ea97285ab5a859b3253
https://gbd.iti.kit.edu/file/a2ca167a8fdbf165f40b5d6dee94b040
https://gbd.iti.kit.edu/file/a5507d7a8dacbc0ecd7abc8d631c266a
https://gbd.iti.kit.edu/file/a5d5c8ce132b1fc0e11ccd557c61bd5c
https://gbd.iti.kit.edu/file/a5e35f56e307070fc85c7dfadacd377e
https://gbd.iti.kit.edu/file/a5fc113e7d4899f4e4af14b87b6fd6ae
https://gbd.iti.kit.edu/file/a81936d2c4de462ab55b6e4660f6f68d
https://gbd.iti.kit.edu/file/a91177361e567d8d16ecb7b847471860
https://gbd.iti.kit.edu/file/a977b9a5632108a51c303a648144f5e9
https://gbd.iti.kit.edu/file/ab3dfe46aee7bfeb8301de497f36a05a
https://gbd.iti.kit.edu/file/abf56d02b82b68d7b372f8a0eaaeb1f3
https://gbd.iti.kit.edu/file/ac6578650f8c5211fbaa1f86a46ccf6e
https://gbd.iti.kit.edu/file/ae17bcd042e6140aa72a2cd9f4c42b9c
https://gbd.iti.kit.edu/file/aef139dd6fc121fbc2d23e43cf88d32b
https://gbd.iti.kit.edu/file/aff1b7e42aafff200fb43c5654004d72
https://gbd.iti.kit.edu/file/b01346439155f2e815fe3754eccdc76c
https://gbd.iti.kit.edu/file/b0384906155607b261b2fb070399ee2e
https://gbd.iti.kit.edu/file/b2a456f73623145ce5a839938aae5d8c
https://gbd.iti.kit.edu/file/b34924ea0812ea17683047a443b6ab99
https://gbd.iti.kit.edu/file/b37dd84a1ca56a24876590bd60c5cf57
https://gbd.iti.kit.edu/file/b3d4854cd2fe7e9f528a6fe75d727bad
https://gbd.iti.kit.edu/file/b4457e1a0929d236a86db88d22e5a31c
https://gbd.iti.kit.edu/file/b5247278fe70f2a5ec06822a78bce5f0
https://gbd.iti.kit.edu/file/b5d9a62d4b2f44b9eb3f6b3e48083ef4
https://gbd.iti.kit.edu/file/b6455805d492bec6e8b3c33f0641e284
https://gbd.iti.kit.edu/file/b64f7d56f1b369fac6a3bcb1efd784da
https://gbd.iti.kit.edu/file/b676ad852ca0867aef85be90753e5745
https://gbd.iti.kit.edu/file/b71310d81f17312ecd887797456b0fb7
https://gbd.iti.kit.edu/file/b7770ebc4d821c53130da4183a3dae50
https://gbd.iti.kit.edu/file/b7f30726f549bf71492647d7e386c1f3
https://gbd.iti.kit.edu/file/b8a14a939eb373007308e61e44db9da5
https://gbd.iti.kit.edu/file/b9733445f4c40231a91fa35523e4163d
https://gbd.iti.kit.edu/file/ba71e5c87a5d9db532d1c60299dea50c
https://gbd.iti.kit.edu/file/ba8621490e4e7212fffdb55fb6bd282d
https://gbd.iti.kit.edu/file/bb12ece2fc8c01870418908a3dd8341f
https://gbd.iti.kit.edu/file/bb15e9530d42642f3f9b9551364d4b32
https://gbd.iti.kit.edu/file/bb2743e5b5dc4987483efc0b1e7572ad
https://gbd.iti.kit.edu/file/bc3c20b0d585bbea7c6d97d4ae056fc4
https://gbd.iti.kit.edu/file/bcdd0116493e34c147aeb8434c6f1109
https://gbd.iti.kit.edu/file/bd39f27a6ff74a059b318d302c9ba2ee
https://gbd.iti.kit.edu/file/bf4a159e317b957da7981146493159e1
https://gbd.iti.kit.edu/file/c2cd624b23b64a57dce567bd50b16965
https://gbd.iti.kit.edu/file/c3c80c8b7e9a4eb4c99be20bee4e2c48
https://gbd.iti.kit.edu/file/c4640fb558ccbad86694363a933084a5
https://gbd.iti.kit.edu/file/c790cce39a626db94903b2789c570d7e
https://gbd.iti.kit.edu/file/c7c22d99715da67f568629dda7d898eb
https://gbd.iti.kit.edu/file/c82eee3badbc8432dad72d1d575a0ea6
https://gbd.iti.kit.edu/file/c947949387613c61a2553098319d9da1
https://gbd.iti.kit.edu/file/c99fbb392c6adaf2835a455b1818e8e2
https://gbd.iti.kit.edu/file/ca6f385b29f7b9fb474f9ed7f9f921af
https://gbd.iti.kit.edu/file/caead0e04b6de4365822b25dfd55967f
https://gbd.iti.kit.edu/file/cc1a816e624e9ef4b3d833c5a43d3651
https://gbd.iti.kit.edu/file/cdd131110acc861a5a01fae6c4936c91
https://gbd.iti.kit.edu/file/cdd89d1b9259dcf26d4a53ba94041e93
https://gbd.iti.kit.edu/file/d08c3d55c88b3c2577800b544399553e
https://gbd.iti.kit.edu/file/d1a2352797c89d56f217e8f8ddc64c82
https://gbd.iti.kit.edu/file/d25b135838aaa1f8e4fdb8d3a7bd9006
https://gbd.iti.kit.edu/file/d2928758b3edd17ebde60f158c520442
https://gbd.iti.kit.edu/file/d36a3588adea5b97fa1cdd2ecb35c055
https://gbd.iti.kit.edu/file/d39f5c2474124910a607eb378b9f0bc3
https://gbd.iti.kit.edu/file/d3a1b763497b09902fd0a3c6afadae63
https://gbd.iti.kit.edu/file/d5d2c97e5135a7468d11a6e1aca5121c
https://gbd.iti.kit.edu/file/d60f56735dc8e89d7d91a01f4340df0c
https://gbd.iti.kit.edu/file/d72b0684dd662ca6475f6caee6a1b450
https://gbd.iti.kit.edu/file/d8e3290dab78fcd69c92a73c03f70630
https://gbd.iti.kit.edu/file/d95c2aa334c098b0a41e0f96ab15cd1d
https://gbd.iti.kit.edu/file/d96b51b728856b588991af6bb20bdde1
https://gbd.iti.kit.edu/file/d9edec809b84b6a3ce31e623026888bc
https://gbd.iti.kit.edu/file/d9f5fe87dfb9d09fab79247f1a5b4e38
https://gbd.iti.kit.edu/file/da7a564a661110a17274c2082c83d5de
https://gbd.iti.kit.edu/file/dac6f7f51d4aad660422a31ed0ee2456
https://gbd.iti.kit.edu/file/db267ea0279158fb3ef0558b73506089
https://gbd.iti.kit.edu/file/dba368a504f627a9fb95cf0b65b512ea
https://gbd.iti.kit.edu/file/dcdfb7b6a5e319f915449dcb2464304d
https://gbd.iti.kit.edu/file/dcea07b4545f04dd2db6b6f6c8be6826
https://gbd.iti.kit.edu/file/ddc308b079b032211ac8e4ae136c2626
https://gbd.iti.kit.edu/file/ddeff6f803b4d0dbb79c841589d3fff9
https://gbd.iti.kit.edu/file/ddfd170730b58b283d98518477b52fe5
https://gbd.iti.kit.edu/file/dea81beed9d7eb7447d94ae7db0b1842
https://gbd.iti.kit.edu/file/deb76f54ab18eb8586d8219bcd61f0f9
https://gbd.iti.kit.edu/file/df050f2e70ef41fc19d87f51ad431601
https://gbd.iti.kit.edu/file/df4c86b3a6551fa9f72a188b0701bea5
https://gbd.iti.kit.edu/file/df8884c93686aa1f0fe4ba12b5a98bde
https://gbd.iti.kit.edu/file/e01c39b7b41c9ff1621579b747254fab
https://gbd.iti.kit.edu/file/e13905790d8d6e35999ee153f50bbdee
https://gbd.iti.kit.edu/file/e207ba2df64495327f715e08155a220f
https://gbd.iti.kit.edu/file/e2d2b011b0805782df6adba648db92e8
https://gbd.iti.kit.edu/file/e3554a7be21336aa38a5fd7c7a1c9261
https://gbd.iti.kit.edu/file/e47010231e43a6d68519a57c386a5509
https://gbd.iti.kit.edu/file/e4c32dcfef4f80078111484c8eed84fd
https://gbd.iti.kit.edu/file/e6015c934b9e0ec60d2debb8f3779057
https://gbd.iti.kit.edu/file/e7aea8faf31ba0ab03bc269223a5bd9d
https://gbd.iti.kit.edu/file/e7b868b93a16feaa7c21bf075cda87f7
https://gbd.iti.kit.edu/file/e8466aed73739b7d020a52c81a4ebad2
https://gbd.iti.kit.edu/file/e85b6cb3e2751d5c80559433ba1adf06
https://gbd.iti.kit.edu/file/e87b1032f86cd784eba018eb8c6358de
https://gbd.iti.kit.edu/file/e8a188835a2623fb53f68b374a18edf0
https://gbd.iti.kit.edu/file/ea88c8ccc24b28ff26c4c82bc956ab4b
https://gbd.iti.kit.edu/file/ead8aa0e85eff93a71d805067f106d07
https://gbd.iti.kit.edu/file/ed121f2f4679309b609ba475ccb79509
https://gbd.iti.kit.edu/file/eda21c1adf5ffb7da982e844f3405982
https://gbd.iti.kit.edu/file/edef62a006514cf5ded0da277c1ecd94
https://gbd.iti.kit.edu/file/eea33de69db92b57462278f766c19787
https://gbd.iti.kit.edu/file/ef83601257510b39572641f2ba485470
https://gbd.iti.kit.edu/file/efe7b285f4605a8d24de73590f358150
https://gbd.iti.kit.edu/file/f13c4fa1ce7c637d1edab07aac5ba030
https://gbd.iti.kit.edu/file/f2141cde748f165bac48de382470d993
https://gbd.iti.kit.edu/file/f26488d372f40f324eda70753bcc66ec
https://gbd.iti.kit.edu/file/f32bb347996c351bc3e9a91c58e8601d
https://gbd.iti.kit.edu/file/f33b2b99b991e02e7c9729fc3f336aac
https://gbd.iti.kit.edu/file/f44b3d7c699c94a24a017aa34c1a058e
https://gbd.iti.kit.edu/file/f4ca5fca83e870d2ed56f2b00b074ed4
https://gbd.iti.kit.edu/file/f50508c7983eea7cce9556409c972877
https://gbd.iti.kit.edu/file/f545c22ae3510cdf7ac578872c7701e4
https://gbd.iti.kit.edu/file/f562b25b05e80a222358bdff30a39a23
https://gbd.iti.kit.edu/file/f59dd3016fb4352fe812aaf6b4dc020b
https://gbd.iti.kit.edu/file/f86dad4ba35369eb720a0c9ddc45037a
https://gbd.iti.kit.edu/file/f8a4580103305bc96f774a186054d322
https://gbd.iti.kit.edu/file/fb79263d8e6bba39c3b3476ffff425d1
https://gbd.iti.kit.edu/file/fcbef8eb7d8cd4f76fc455a25c0d3065
https://gbd.iti.kit.edu/file/fee70cede2b5b55bfbdb6e48fbe7ce4f
https://gbd.iti.kit.edu/file/c5e7e5564e9c70329212c6bd040f30ba
https://gbd.iti.kit.edu/file/d55033d9dc6c92861ae6b099870322d3
https://gbd.iti.kit.edu/file/fb852d4b14865a562c1a38a77c8517d1
https://gbd.iti.kit.edu/file/024af9416f8c1dad1b4f974757e38d51
https://gbd.iti.kit.edu/file/042846b7b23a617ad1730e9eb2dbdceb
https://gbd.iti.kit.edu/file/049072033960eab882a3b0933d414f36
https://gbd.iti.kit.edu/file/06872e73223dc74c76b06f37a185dc22
https://gbd.iti.kit.edu/file/0900255c001ced2b14a6cadc0403967e
https://gbd.iti.kit.edu/file/0a11a3dcc342fba24aeda274ad56d478
https://gbd.iti.kit.edu/file/1364901fb87afe648b513e945ddeed91
https://gbd.iti.kit.edu/file/1401185fe7e0a5f1139ff4755d65a089
https://gbd.iti.kit.edu/file/172ecb98a80b859e62612ff192a53729
https://gbd.iti.kit.edu/file/17b54dc4971c1a5f907b5b9de801c1c6
https://gbd.iti.kit.edu/file/1a3400dc32a3859afb20566a4ded9606
https://gbd.iti.kit.edu/file/1bb9fe12b4fb50e98903ee87a51d6472
https://gbd.iti.kit.edu/file/1c3ead67cae5d59d980d770049594c12
https://gbd.iti.kit.edu/file/1e35d939af7897ca53f87757754adec9
https://gbd.iti.kit.edu/file/1f413718bf393d43d31a1e498fc4d123
https://gbd.iti.kit.edu/file/2115182958f6cfb5a172a24f989b86dd
https://gbd.iti.kit.edu/file/245e9b5cdf197b523d57f35ac2fcb303
https://gbd.iti.kit.edu/file/268786234185b8a93051bee4efaad466
https://gbd.iti.kit.edu/file/27f890d3aa346aaa55a31b63ebd4952c
https://gbd.iti.kit.edu/file/2aab943d5ee5d7b3888f1ca30a453c4f
https://gbd.iti.kit.edu/file/2c7e530bd383ed94fd6da30cbe5d146f
https://gbd.iti.kit.edu/file/2dc39ea808c5fffc08d2ab8c48881c80
https://gbd.iti.kit.edu/file/2e0d0b143cff2105aa07d57e30ea5d74
https://gbd.iti.kit.edu/file/3354c5f91c523cfc42dab6a40129d102
https://gbd.iti.kit.edu/file/3454285436b59c94031a9d8831dae833
https://gbd.iti.kit.edu/file/3b5f922e28a18ed17be03f0fd3923ce8
https://gbd.iti.kit.edu/file/3fac9d4ad62698423574608cd2c44795
https://gbd.iti.kit.edu/file/4165a3f519e2381da2f25f43cd325d3f
https://gbd.iti.kit.edu/file/446faa6735e7611dddf64e64162d0156
https://gbd.iti.kit.edu/file/4a06483dc891f16c60ee8095e982b455
https://gbd.iti.kit.edu/file/4a3bee9d892a695c3d63191f4d1fbdff
https://gbd.iti.kit.edu/file/4dd165fbd1678001e130c89cfa2969a2
https://gbd.iti.kit.edu/file/4dfe7816c2c198f8fd0b328d1e9672c9
https://gbd.iti.kit.edu/file/4f888358cd62fe7bc602ea6882cdfc6d
https://gbd.iti.kit.edu/file/529368029977c0f315a703cce818eb55
https://gbd.iti.kit.edu/file/5553fec254b32349e0e48c7ee1e2d6c7
https://gbd.iti.kit.edu/file/562c4f75b56aa462a9e77c787b9bdeca
https://gbd.iti.kit.edu/file/5880595d26c032ea3f58c8bfacf19754
https://gbd.iti.kit.edu/file/5a0120a44efe84c0cb4e58cebc3c4982
https://gbd.iti.kit.edu/file/5a0b7708bd8eb4c1d26ea32e56bf47cf
https://gbd.iti.kit.edu/file/6147e666b75f603a4c4490d21ab654cd
https://gbd.iti.kit.edu/file/687d3ac645f05ea381091ab1536bd24a
https://gbd.iti.kit.edu/file/6b6cab90664478877c3a63248c9fb961
https://gbd.iti.kit.edu/file/6fc6c472b92f70f845c8d75c8268d44a
https://gbd.iti.kit.edu/file/7238c9b56aecfad5079a5c20fa363185
https://gbd.iti.kit.edu/file/744199dff0b9d2fc6f3f5b73d0f924c1
https://gbd.iti.kit.edu/file/7882d223c35d697cb9e069b37bac4349
https://gbd.iti.kit.edu/file/7a057dad354465237f3e14900e400860
https://gbd.iti.kit.edu/file/7a700945b9efe7d98a64e6d5da3c1aa7
https://gbd.iti.kit.edu/file/7bcbe64c5bdad9b53ca0aae0000e3bcf
https://gbd.iti.kit.edu/file/7ec0edbb9a103444312504dfd67e0e72
https://gbd.iti.kit.edu/file/897c9f5f6396aba631f6e78e675028a8
https://gbd.iti.kit.edu/file/8b31606e10656ff7eb2936262b647443
https://gbd.iti.kit.edu/file/8bf857909f1117a9e29a4788bf77d485
https://gbd.iti.kit.edu/file/9421f6e82c45ee7b58d5bcbe860ca3cc
https://gbd.iti.kit.edu/file/976649ddfe27c60bacb508ecd3499f39
https://gbd.iti.kit.edu/file/99ad93fc5e796a2e617e420f907aaa88
https://gbd.iti.kit.edu/file/9e4ca1fd351c9bcdd837bd19f96a25bf
https://gbd.iti.kit.edu/file/a130a3f95891daed1bfd71c77bd2d8a1
https://gbd.iti.kit.edu/file/adeb4b7acbe99e55349d830bcf7da6bc
https://gbd.iti.kit.edu/file/b14723fd0acbb7a5b497c4ddb4e3d813
https://gbd.iti.kit.edu/file/b400f2362d15334aa6d05ef99e315fc1
https://gbd.iti.kit.edu/file/b51583e32432c5778c7e3e996c3bfeba
https://gbd.iti.kit.edu/file/b55b472a43a2daa68378ac8cbc4c2fb4
https://gbd.iti.kit.edu/file/b5df8842330d20ec7102405549d282ed
https://gbd.iti.kit.edu/file/ba5451cace1bb09e2d36b9f34c19146e
https://gbd.iti.kit.edu/file/bd3f31c8c7ee0810b9853a88eb626b81
https://gbd.iti.kit.edu/file/be805e2615367977aa2ef37681a01956
https://gbd.iti.kit.edu/file/bf31eea5a8b87410f1fb83064e2cb82e
https://gbd.iti.kit.edu/file/c1b5c6d416294690d5be8971d53fc8b7
https://gbd.iti.kit.edu/file/c1c5f27a2c47486a745e7aa3516fb24d
https://gbd.iti.kit.edu/file/c39ff9218232adb739cd31f4c70fb316
https://gbd.iti.kit.edu/file/c5de36538149b595c16b39ed0940958f
https://gbd.iti.kit.edu/file/c602a712213cb8bb2d132bf217a430b9
https://gbd.iti.kit.edu/file/c76f3c6c2e2eae85fc5f3d499f3db88d
https://gbd.iti.kit.edu/file/c8427b1218785dcf28539e220f800864
https://gbd.iti.kit.edu/file/cd864fc43dfa07c29d9c6201d107160b
https://gbd.iti.kit.edu/file/ceb4b1db49b77dbd68fa63ce029490c1
https://gbd.iti.kit.edu/file/cec5d86e0eae8234ca7c29b31c4674a6
https://gbd.iti.kit.edu/file/d0c14f10c8b46bd94bd849c661c0a3e5
https://gbd.iti.kit.edu/file/db480ef956c6e29f7aa1b4ba155cfa75
https://gbd.iti.kit.edu/file/db5739060cc056b6b4733e4527bb630c
https://gbd.iti.kit.edu/file/dcfdf31943b65080a2e4b8ec46a89c7d
https://gbd.iti.kit.edu/file/dddf7932f1305a4dc37ba81d0a5c6889
https://gbd.iti.kit.edu/file/e4693381ca70fb32a9e560279156afcc
https://gbd.iti.kit.edu/file/e47586f86868e83a5ebe7a3ecae204f1
https://gbd.iti.kit.edu/file/e7a41d4d4a3a68de583fa98376bf43f2
https://gbd.iti.kit.edu/file/eb18bb51407d6d53f2e48f8938840dfa
https://gbd.iti.kit.edu/file/f3bda5fcae82cfa27460df877965eeca
https://gbd.iti.kit.edu/file/f421f53b4a01716170fd51e614b58d4c
https://gbd.iti.kit.edu/file/f67c5855cfebd80998ab524229af16ae
https://gbd.iti.kit.edu/file/f70ef02944377b624faa46b14132983c
https://gbd.iti.kit.edu/file/faea4b579b0010b32f001ac4b5b9415a
https://gbd.iti.kit.edu/file/fc538412229f86bc7016b8bc0aca3924
https://gbd.iti.kit.edu/file/fc794887d3aede5fdd422a501f730032
https://gbd.iti.kit.edu/file/ff0d5cb434081622290325ab3ed57fd7
https://gbd.iti.kit.edu/file/0f4911e1180b80b45e9db7ec5e73eb66
https://gbd.iti.kit.edu/file/9aaadc9d5e63938b3711ea6689579055
https://gbd.iti.kit.edu/file/af66c2b2ff8a9cd900d9f2f79e53f6a7
https://gbd.iti.kit.edu/file/b26e88b9b0d4b6f9e6ae95efe0b8df02\
";
