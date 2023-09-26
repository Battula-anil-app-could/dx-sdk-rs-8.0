use dharitri_sc_scenario::*;

const WMOA_SWAP_EXPR: &str = "0x0061736d0100000001661160000060017f0060027f7f017f60027f7f006000017f60017f017f60037f7f7f0060037f7f7f017f60057f7f7e7f7f017f60047f7f7f7f0060067e7f7f7f7f7f017f60047f7f7f7f017f60027f7e0060017e006000017e60057f7f7f7f7f0060037e7f7f0002cc062303656e760b7369676e616c4572726f72000303656e7618626967496e7447657445787465726e616c42616c616e6365000303656e760a6d4275666665724e6577000403656e760d6d427566666572417070656e64000203656e76096d4275666665724571000203656e76106d616e61676564534341646472657373000103656e761b6d616e61676564457865637574654f6e44657374436f6e74657874000a03656e760f636c65616e52657475726e44617461000003656e760d6d616e6167656443616c6c6572000103656e76136d616e616765644f776e657241646472657373000103656e76126d427566666572476574417267756d656e74000203656e760f6765744e756d417267756d656e7473000403656e76126d427566666572417070656e644279746573000703656e760f6d4275666665725365744279746573000703656e76106d4275666665724765744c656e677468000503656e76196d42756666657246726f6d426967496e74556e7369676e6564000203656e76136d42756666657247657442797465536c696365000b03656e76126d42756666657253746f726167654c6f6164000203656e76126d616e616765645369676e616c4572726f72000103656e760e626967496e74536574496e743634000c03656e760a626967496e745369676e000503656e76136d42756666657253746f7261676553746f7265000203656e760e636865636b4e6f5061796d656e74000003656e7614626967496e7446696e697368556e7369676e6564000103656e760d6d42756666657246696e697368000503656e7614736d616c6c496e7446696e6973685369676e6564000d03656e761c6d616e616765644765744d756c74694553445443616c6c56616c7565000103656e7609626967496e74436d70000203656e760a6765744761734c656674000e03656e761b6d616e616765645472616e7366657256616c756545786563757465000803656e76136765744e756d455344545472616e7366657273000403656e7612626967496e7447657443616c6c56616c7565000103656e7609626967496e74416464000603656e76226d616e616765644d756c74695472616e73666572455344544e465445786563757465000803656e760f6d42756666657247657442797465730002032c2b0f00040405020310010400010303040503020301050509030306060900040501000000000000000000000005030100110619037f01418080c0000b7f0041e5d1c0000b7f0041f0d1c0000b079b010c066d656d6f7279020004696e69740044146765744c6f636b656445676c6442616c616e63650045156765745772617070656445676c64546f6b656e496400460869735061757365640047057061757365004807756e706175736500490a756e7772617045676c64004a087772617045676c64004b0863616c6c4261636b004c0a5f5f646174615f656e6403010b5f5f686561705f6261736503020abc162b2e000240200120024d0440200220044d0d011024000b1024000b2000200220016b3602042000200120036a3602000b05001043000b2301027f10262200100520001026210041c5d1c00010221a41c5d1c0002000100120000b1b01017f41a483c00041a483c00028020041016b220036020020000b0f01017f10022201200010031a20010b0b0020002001100441004a0b0900200020011000000b1f01017f4167100510262203102b20004167200320012002102610061a10070b08002000420010130b0c01017f10262200100820000b1e01017f1026220010092000102c102804400f0b41f082c00041241000000b1500100b20004604400f0b41cb81c00041191000000b4701017f230041106b2202240020022001410874418080fc077120014118747220014108764180fe03712001411876727236020c20002002410c6a4104100c1a200241106a24000b0d0010311a200020011032102f0b1401017f1026220041d482c0004100100d1a20000b0f01017f102622012000100f1a20010b0d0010311a200020011027102f0b1101017f1026220220002001100d1a20020b09002000200110031a0b5501017f20002d00042101200041003a000402402001410171044041a883c00028020022014191ce004f0d01200028020041ac83c0002001100c1a41a883c000410036020041bcd1c00041003a00000b0f0b1024000bae0102027f017e230041106b2202240020024200370308200010382200100e220141094904402002200241086a41082001103920004100200228020422002002280200220110101a027f41002000450d001a034020000440200041016b210020013100002003420886842103200141016a21010c010b0b02402003420158044041002003a741016b0d021a0c010b41f281c0004112103a000b41010b200241106a24000f0b41e481c000410e103a000b0d0020001026220010111a20000b3b01017f230041106b22042400200441086a41002003200120021023200428020c21012000200428020836020020002001360204200441106a24000b1b01017f418482c00041161034220220002001100c1a20021012000bf20101047f230041106b2203240020032000100e22024118742002410874418080fc07717220024108764180fe03712002411876727236020c20012003410c6a4104103c20012d00042102200141003a00040240024002402002410171220204402000100e22054190ce0041a883c00028020022046b4b0d0220032004200420056a2204103d200041002003280204200328020010101a41a883c00020043602000c010b2001280200200010350b200120023a00040c010b2001103620012802002000103520012d0004200120023a0004410171450d0041a883c000410036020041bcd1c00041003a00000b200341106a24000b800101027f230041106b220324000240024020002d000404404190ce0041a883c00028020022046b2002490d01200341086a2004200220046a2200103d2003280208200328020c20012002103e41a883c00020003602000c020b200028020020012002100c1a0c010b20001036200028020020012002100c1a0b200341106a24000b4001017f230041106b22032400200341086a2001200241ac83c0004190ce001023200328020c21012000200328020836020020002001360204200341106a24000bb00201067f2001200346044020012203410f4b04402000410020006b41037122056a210420050440200221010340200020012d00003a0000200141016a2101200041016a22002004490d000b0b2004200320056b2203417c7122066a21000240200220056a220541037122010440200641004c0d012005417c71220741046a21024100200141037422086b4118712109200728020021010340200420012008762002280200220120097472360200200241046a2102200441046a22042000490d000b0c010b200641004c0d0020052102034020042002280200360200200241046a2102200441046a22042000490d000b0b20034103712103200520066a21020b20030440200020036a21010340200020022d00003a0000200241016a2102200041016a22002001490d000b0b0f0b1043000b1c0041be82c0004113103410374504400f0b419a82c00041121029000b0b0041ac82c000411210340b09002000101441004a0ba60101037f230041106b2201240041be82c00041131034200142808080808080808001420020001b3703080240200045044041d482c00021000c010b4100210003400240024020004108470440200141086a20006a2d00002202450d022002411874411f7520006a220041094f0d01410820006b21022000200141086a6a21000c040b1043000b104d000b200041016a21000c000b000b20002002103410151a200141106a24000b0500104d000b1a01017f10164101102e410010262200100a1a1040200010151a0b0c0010164100102e102510170b0f0010164100102e1040103810181a0b160010164100102e41be82c000411310341037ad10190b0e001016102d4100102e410110420b0e001016102d4100102e410010420bed0302097f017e230041106b220124004100102e103f416b2102024041c4d1c0002d000022000440416b41ffffffff0720001b21020c010b41c4d1c00041013a0000416b101a0b02402002100e4170714110460440410021002002100e2106200141086a2107410121050340200041106a220420064b0d022007420037030020014200370300200220004110200110101a2005044020012902042209423886200942288642808080808080c0ff0083842009421886428080808080e03f8320094208864280808080f01f838484200942088842808080f80f832009421888428080fc07838420094228884280fe038320094238888484842109200128020022004118742000410874418080fc07717220004108764180fe0371200041187672722108200128020c22004118742000410874418080fc07717220004108764180fe037120004118767272210341002105200421000c010b0b1043000b418481c00041221000000b024002400240200950044020081040103822041028450d0120031041450d0220031025101b41004a0d031031220020041033200020031030101c41ea80c000410d10342000102a102c2003420010311031101d1a200141106a24000f0b41d482c000411c1000000b418080c00041101029000b419080c000411c1029000b41ac80c00041231029000beb0301097f230041206b220124000240101e4504404100102e103f41752103024041c0d1c0002d000022000440417541ffffffff0720001b21030c010b41c0d1c00041013a00004175101f0b20031041450d011040103821041031220020041033200020031030101c41f780c000410d10342000102a102c10312107103121081031210520041027210210262200102b20002000200310202001420037020c20012002410874418080fc077120024118747220024108764180fe03712002411876727236020820012000410874418080fc077120004118747220004108764180fe0371200041187672723602142005200141086a4110100c1a200542002007200810211a2001027f41bcd1c0002d0000220045044041bcd1c00041013a000041a883c0004100360200200141ac83c0004190ce00410010392001280200200128020441d482c0004100103e10310c010b41d482c000410010340b360218200120004101733a001c2004200141186a2200103b200142003703082000200141086a22024108103c200310322000103b20012802182100200120012d001c3a000c2001200036020820021036200128020820012d000c044041a883c000410036020041bcd1c00041003a00000b10181a200141206a24000f0b41a681c00041251000000b41cf80c000411b1029000b0300010b0c00419483c000410e1000000b0bb8030200418080c0000ba20357726f6e67206573647420746f6b656e4d75737420706179206d6f7265207468616e203020746f6b656e7321436f6e747261637420646f6573206e6f74206861766520656e6f7567682066756e64735061796d656e74206d757374206265206d6f7265207468616e2030455344544c6f63616c4275726e455344544c6f63616c4d696e74696e636f7272656374206e756d626572206f662045534454207472616e736665727366756e6374696f6e20646f6573206e6f74206163636570742045534454207061796d656e7477726f6e67206e756d626572206f6620617267756d656e7473696e70757420746f6f206c6f6e67696e707574206f7574206f662072616e676573746f72616765206465636f6465206572726f723a20436f6e7472616374206973207061757365647772617070656445676c64546f6b656e496470617573655f6d6f64756c653a70617573656400000066756e6769626c65204553445420746f6b656e206578706563746564456e64706f696e742063616e206f6e6c792062652063616c6c6564206279206f776e657270616e6963206f636375727265640041a483c0000b049cffffff";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig.wasm",
        multisig::ContractBuilder,
        "multisig",
    );
    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig-view.wasm",
        multisig::ContractBuilder,
        "multisig-view",
    );

    blockchain.register_contract("file:test-contracts/adder.wasm", adder::ContractBuilder);

    blockchain.register_contract(
        "file:test-contracts/factorial.wasm",
        factorial::ContractBuilder,
    );

    blockchain.register_contract(WMOA_SWAP_EXPR, dharitri_wmoa_swap_sc::ContractBuilder);

    blockchain
}

#[test]
#[ignore]
fn call_other_shard_1_rs() {
    world().run("scenarios/call_other_shard-1.scen.json");
}

#[test]
#[ignore]
fn call_other_shard_2_rs() {
    world().run("scenarios/call_other_shard-2.scen.json");
}

#[test]
fn change_board_rs() {
    world().run("scenarios/changeBoard.scen.json");
}

#[test]
fn change_quorum_rs() {
    world().run("scenarios/changeQuorum.scen.json");
}

#[test]
fn change_quorum_too_big_rs() {
    world().run("scenarios/changeQuorum_tooBig.scen.json");
}

#[test]
fn deploy_adder_err_rs() {
    world().run("scenarios/deployAdder_err.scen.json");
}

#[test]
fn deploy_adder_then_call_rs() {
    world().run("scenarios/deployAdder_then_call.scen.json");
}

#[test]
fn deploy_factorial_rs() {
    world().run("scenarios/deployFactorial.scen.json");
}

#[test]
fn deploy_other_multisig_rs() {
    world().run("scenarios/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_rs() {
    world().run("scenarios/deploy_duplicate_bm.scen.json");
}

#[test]
fn interactor_nft_rs() {
    world().run("scenarios/interactor_nft.scen.json");
}

#[test]
fn interactor_nft_all_roles_rs() {
    world().run("scenarios/interactor_nft_all_roles.scen.json");
}

#[test]
fn interactor_wmoa_rs() {
    world().run("scenarios/interactor_wmoa.scen.json");
}

#[test]
fn remove_everyone_rs() {
    world().run("scenarios/remove_everyone.scen.json");
}

#[test]
fn send_dct_rs() {
    world().run("scenarios/sendDct.scen.json");
}

#[test]
fn upgrade_rs() {
    world().run("scenarios/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_rs() {
    world().run("scenarios/upgrade_from_source.scen.json");
}
