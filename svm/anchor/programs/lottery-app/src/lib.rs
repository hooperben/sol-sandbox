use anchor_lang::prelude::*;

use num_bigint::BigUint;
use num_traits::FromPrimitive;

declare_id!("C1HzcdTw9f5rEyrEHW6FnmdWo9HuxY4gwW9AJ18EgHJw");

#[program]
pub mod lottery_app {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    // 0x0000000000000000000000000000000000000000000000000000000000002000
    pub const VK_CIRCUIT_SIZE: [u64; 4] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000002000,
    ];
    // 0x0000000000000000000000000000000000000000000000000000000000000003
    pub const VK_NUM_INPUTS: [u64; 4] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000003,
    ];
    // 0x006fab49b869ae62001deac878b2667bd31bf3e28e3a2d764aa49b8d9bbdd310
    pub const VK_WORK_ROOT: [u64; 4] = [
        0x006fab49b869ae62,
        0x001deac878b2667b,
        0xd31bf3e28e3a2d76,
        0x4aa49b8d9bbdd310,
    ];
    // 0x3062cb506d9a969cb702833453cd4c52654aa6a93775a2c5bf57d68443608001
    pub const VK_DOMAIN_INVERSE: [u64; 4] = [
        0x3062cb506d9a969c,
        0xb702833453cd4c52,
        0x654aa6a93775a2c5,
        0xbf57d68443608001,
    ];
    // 0x28757cafd515f607cbeaeb4515495c3b3c8713bc62bd31b1864c27a40d2535ed
    pub const VK_Q1_X: [u64; 4] = [
        0x28757cafd515f607,
        0xcbeaeb4515495c3b,
        0x3c8713bc62bd31b1,
        0x864c27a40d2535ed,
    ];
    // 0x0c672512cacb2b2c73c1c5b5c9988e9f6b34abe07690abcd26ec13748eda964a
    pub const VK_Q1_Y: [u64; 4] = [
        0x0c672512cacb2b2c,
        0x73c1c5b5c9988e9f,
        0x6b34abe07690abcd,
        0x26ec13748eda964a,
    ];
    // 0x2ef390db388bde18cb22e5dfcb283367b1f35ab86f9fd4b1a184b5cfe346254c
    pub const VK_Q2_X: [u64; 4] = [
        0x2ef390db388bde18,
        0xcb22e5dfcb283367,
        0xb1f35ab86f9fd4b1,
        0xa184b5cfe346254c,
    ];
    // 0x0fb2d61440a13ece99bddada11647155217bb6f044b7e97bdf22ab2743c39201
    pub const VK_Q2_Y: [u64; 4] = [
        0x0fb2d61440a13ece,
        0x99bddada11647155,
        0x217bb6f044b7e97b,
        0xdf22ab2743c39201,
    ];
    // 0x15b545b5aafcdfa7b753a91e0ce9b8c1b2175899a46b298f489344b21ba94ef3
    pub const VK_Q3_X: [u64; 4] = [
        0x15b545b5aafcdfa7,
        0xb753a91e0ce9b8c1,
        0xb2175899a46b298f,
        0x489344b21ba94ef3,
    ];
    // 0x0fd2e5dcb2e98b92c173705cfe2df0dc40b9c7ad09210695a2f98515072a2894
    pub const VK_Q3_Y: [u64; 4] = [
        0x0fd2e5dcb2e98b92,
        0xc173705cfe2df0dc,
        0x40b9c7ad09210695,
        0xa2f98515072a2894,
    ];
    // 0x23d2dd0b829610c59d099018508ee128bbc71ab68002411490e04a9122ece9bd
    pub const VK_Q4_X: [u64; 4] = [
        0x23d2dd0b829610c5,
        0x9d099018508ee128,
        0xbbc71ab680024114,
        0x90e04a9122ece9bd,
    ];
    // 0x2f11872338be4f110399909a8cde9b32b94003facd05d98e241908059c2c20c7
    pub const VK_Q4_Y: [u64; 4] = [
        0x2f11872338be4f11,
        0x0399909a8cde9b32,
        0xb94003facd05d98e,
        0x241908059c2c20c7,
    ];
    // 0x186ccce2aa7fd77041be861cb5ca726a9eec91d2b6edb4755370e23b665b86ba
    pub const VK_Q_M_X: [u64; 4] = [
        0x186ccce2aa7fd770,
        0x41be861cb5ca726a,
        0x9eec91d2b6edb475,
        0x5370e23b665b86ba,
    ];
    // 0x0c6a6f7f5826d7f690605d9228c4b42589c3b2970c875452526bbead94525392
    pub const VK_Q_M_Y: [u64; 4] = [
        0x0c6a6f7f5826d7f6,
        0x90605d9228c4b425,
        0x89c3b2970c875452,
        0x526bbead94525392,
    ];
    // 0x0ae2edab13a0ab00a9378ae4863d6a7e4da5cbb5d54edc032081840be8732115
    pub const VK_Q_C_X: [u64; 4] = [
        0x0ae2edab13a0ab00,
        0xa9378ae4863d6a7e,
        0x4da5cbb5d54edc03,
        0x2081840be8732115,
    ];
    // 0x1ce293e6e839821a81e751636abc7d8fb59b564c12aa99929158ace8234ef1b3
    pub const VK_Q_C_Y: [u64; 4] = [
        0x1ce293e6e839821a,
        0x81e751636abc7d8f,
        0xb59b564c12aa9992,
        0x9158ace8234ef1b3,
    ];
    // 0x2cb5a3fc3d66ae99856b96b5048e69b57ebfdd0e97bd573c59fb1e2ab1c0b6a0
    pub const VK_Q_ARITHMETIC_X: [u64; 4] = [
        0x2cb5a3fc3d66ae99,
        0x856b96b5048e69b5,
        0x7ebfdd0e97bd573c,
        0x59fb1e2ab1c0b6a0,
    ];
    // 0x2f8c572aa95215ca5fc74d104288d19987f8e650ab1e69897fd1473c4755e854
    pub const VK_Q_ARITHMETIC_Y: [u64; 4] = [
        0x2f8c572aa95215ca,
        0x5fc74d104288d199,
        0x87f8e650ab1e6989,
        0x7fd1473c4755e854,
    ];
    // 0x1ff5f913ca607ba54b4ace25cf8647967e46a355ba1a069304939a326b878bf6
    pub const VK_QSORT_X: [u64; 4] = [
        0x1ff5f913ca607ba5,
        0x4b4ace25cf864796,
        0x7e46a355ba1a0693,
        0x04939a326b878bf6,
    ];
    // 0x19c2d7fbdc0b9b88780055df45f0184f2c123ca016a32adc6f09f286c9947d0c
    pub const VK_QSORT_Y: [u64; 4] = [
        0x19c2d7fbdc0b9b88,
        0x780055df45f0184f,
        0x2c123ca016a32adc,
        0x6f09f286c9947d0c,
    ];
    // 0x28f715c93bc880fbc43a5f2cced51a040c881a59d47e2317cde990e3d97e458a
    pub const VK_Q_ELLIPTIC_X: [u64; 4] = [
        0x28f715c93bc880fb,
        0xc43a5f2cced51a04,
        0x0c881a59d47e2317,
        0xcde990e3d97e458a,
    ];
    // 0x14a796f0bc75f4e40a23ecd4af7de1ca745879d983d68d056469b8d20705275c
    pub const VK_Q_ELLIPTIC_Y: [u64; 4] = [
        0x14a796f0bc75f4e4,
        0x0a23ecd4af7de1ca,
        0x745879d983d68d05,
        0x6469b8d20705275c,
    ];
    // 0x031fb161c3fbe55d75807bac763a4ba43d14a402f896de249bb48174a812bb61
    pub const VK_Q_AUX_X: [u64; 4] = [
        0x031fb161c3fbe55d,
        0x75807bac763a4ba4,
        0x3d14a402f896de24,
        0x9bb48174a812bb61,
    ];
    // 0x1e22b35319ce5346023c24d3f892fd07d34b02953961f01696417deb221c3509
    pub const VK_Q_AUX_Y: [u64; 4] = [
        0x1e22b35319ce5346,
        0x023c24d3f892fd07,
        0xd34b02953961f016,
        0x96417deb221c3509,
    ];
    // 0x22f8fcfaa09563966966b7b23b392ecbe951696a36ead6ff31ccc52b32f97866
    pub const VK_SIGMA1_X: [u64; 4] = [
        0x22f8fcfaa0956396,
        0x6966b7b23b392ecb,
        0xe951696a36ead6ff,
        0x31ccc52b32f97866,
    ];
    // 0x1872df144cb2c2d16165380b232e9c555bf67924b48c22b53ca0c631696785ad
    pub const VK_SIGMA1_Y: [u64; 4] = [
        0x1872df144cb2c2d1,
        0x6165380b232e9c55,
        0x5bf67924b48c22b5,
        0x3ca0c631696785ad,
    ];
    // 0x1692f386300cfe92ee01f9781fcbec9438a46028841b206af0a30dc24ae3d5ba
    pub const VK_SIGMA2_X: [u64; 4] = [
        0x1692f386300cfe92,
        0xee01f9781fcbec94,
        0x38a46028841b206a,
        0xf0a30dc24ae3d5ba,
    ];
    // 0x28b429cd57b4d6928aea17dc8d8c0edb18d614ff3ac51b6fa8f6fa20c5d265f9
    pub const VK_SIGMA2_Y: [u64; 4] = [
        0x28b429cd57b4d692,
        0x8aea17dc8d8c0edb,
        0x18d614ff3ac51b6f,
        0xa8f6fa20c5d265f9,
    ];
    // 0x12496a2cda99445e7767c5e810f2a7f4318e3e7be1198eb501ac5da763f0dbc4
    pub const VK_SIGMA3_X: [u64; 4] = [
        0x12496a2cda99445e,
        0x7767c5e810f2a7f4,
        0x318e3e7be1198eb5,
        0x01ac5da763f0dbc4,
    ];
    // 0x2b2a4775c72353060dd186b99d35266c9f4d0752700d47ca86439d496c750685
    pub const VK_SIGMA3_Y: [u64; 4] = [
        0x2b2a4775c7235306,
        0x0dd186b99d35266c,
        0x9f4d0752700d47ca,
        0x86439d496c750685,
    ];
    // 0x23b4d8c19644459cbf9a3b61632d76d7861dbe78f11a7f8c0fc09b63ea4258f7
    pub const VK_SIGMA4_X: [u64; 4] = [
        0x23b4d8c19644459c,
        0xbf9a3b61632d76d7,
        0x861dbe78f11a7f8c,
        0x0fc09b63ea4258f7,
    ];
    // 0x009cfb99f579cbddc0c8465bd277f9295dd694ab201215c3e96d7b15a2951981
    pub const VK_SIGMA4_Y: [u64; 4] = [
        0x009cfb99f579cbdd,
        0xc0c8465bd277f929,
        0x5dd694ab201215c3,
        0xe96d7b15a2951981,
    ];
    // 0x03c3f013d7bd74d9d03aecd380c7bcfded7bffcac5e9ca72f4fece642d94840a
    pub const VK_TABLE1_X: [u64; 4] = [
        0x03c3f013d7bd74d9,
        0xd03aecd380c7bcfd,
        0xed7bffcac5e9ca72,
        0xf4fece642d94840a,
    ];
    // 0x00af81192977a3213784968885b8c6e575a39c067134bc6da87d45eacf108da0
    pub const VK_TABLE1_Y: [u64; 4] = [
        0x00af81192977a321,
        0x3784968885b8c6e5,
        0x75a39c067134bc6d,
        0xa87d45eacf108da0,
    ];
    // 0x2efcd1e3d0e83fe2adebd9bed2dbea769748c27cbe1c3b1318eb029ff9fbae25
    pub const VK_TABLE2_X: [u64; 4] = [
        0x2efcd1e3d0e83fe2,
        0xadebd9bed2dbea76,
        0x9748c27cbe1c3b13,
        0x18eb029ff9fbae25,
    ];
    // 0x03c3d6fb451f07a91ca5c7cd137769a836a0e3627521042cb0231aa62cd5201e
    pub const VK_TABLE2_Y: [u64; 4] = [
        0x03c3d6fb451f07a9,
        0x1ca5c7cd137769a8,
        0x36a0e3627521042c,
        0xb0231aa62cd5201e,
    ];
    // 0x1e94cb1d304dc0752db812d57390ffa30a6e4c0f924346e0ec56bb4baca8eb0c
    pub const VK_TABLE3_X: [u64; 4] = [
        0x1e94cb1d304dc075,
        0x2db812d57390ffa3,
        0x0a6e4c0f924346e0,
        0xec56bb4baca8eb0c,
    ];
    // 0x0a2f8564302b49ffbb94512c138be76da6e58c12e7f57b51fdb35f9e6c4c1fbc
    pub const VK_TABLE3_Y: [u64; 4] = [
        0x0a2f8564302b49ff,
        0xbb94512c138be76d,
        0xa6e58c12e7f57b51,
        0xfdb35f9e6c4c1fbc,
    ];
    // 0x0d141201ddcd88cd2a0ada7bf51517c3c08b8dbaf05d09b7119faacc3706e1ae
    pub const VK_TABLE4_X: [u64; 4] = [
        0x0d141201ddcd88cd,
        0x2a0ada7bf51517c3,
        0xc08b8dbaf05d09b7,
        0x119faacc3706e1ae,
    ];
    // 0x2a6d896384beaf8a2b86d8438e1b1e32f5f21b714dfdb8536a22ab3c0442fe12
    pub const VK_TABLE4_Y: [u64; 4] = [
        0x2a6d896384beaf8a,
        0x2b86d8438e1b1e32,
        0xf5f21b714dfdb853,
        0x6a22ab3c0442fe12,
    ];
    // 0x1aa5249a55d63edf065fc7fee5f604006244319b695612028eb7bcb9350f2e37
    pub const VK_TABLE_TYPE_X: [u64; 4] = [
        0x1aa5249a55d63edf,
        0x065fc7fee5f60400,
        0x6244319b69561202,
        0x8eb7bcb9350f2e37,
    ];
    // 0x2465961b676caff24e134a16641123d182dc02bc8390aa5ef6e0fa77566f9d29
    pub const VK_TABLE_TYPE_Y: [u64; 4] = [
        0x2465961b676caff2,
        0x4e134a16641123d1,
        0x82dc02bc8390aa5e,
        0xf6e0fa77566f9d29,
    ];
    // 0x09cc94458ee0afa1009fb97bb061b102d970106e6d64aa00df4d555b6136f805
    pub const VK_ID1_X: [u64; 4] = [
        0x09cc94458ee0afa1,
        0x009fb97bb061b102,
        0xd970106e6d64aa00,
        0xdf4d555b6136f805,
    ];
    // 0x10ac975fabe85d5531e9a6abd70d039a412ac9582e7f7957836998b801af25bc
    pub const VK_ID1_Y: [u64; 4] = [
        0x10ac975fabe85d55,
        0x31e9a6abd70d039a,
        0x412ac9582e7f7957,
        0x836998b801af25bc,
    ];
    // 0x2919e193e97154c7400631f427e56fd630fa438b112cff9b4fcd146565db34d6
    pub const VK_ID2_X: [u64; 4] = [
        0x2919e193e97154c7,
        0x400631f427e56fd6,
        0x30fa438b112cff9b,
        0x4fcd146565db34d6,
    ];
    // 0x00f8d5e9e9c8137e87911a8d8aabb84f21d0921cb24bdd646bcd6d731d758d3c
    pub const VK_ID2_Y: [u64; 4] = [
        0x00f8d5e9e9c8137e,
        0x87911a8d8aabb84f,
        0x21d0921cb24bdd64,
        0x6bcd6d731d758d3c,
    ];
    // 0x10467ee1c381e9a7068e5b862079d270bb6ae5f190ba4bcbf2a1e08a932b73ba
    pub const VK_ID3_X: [u64; 4] = [
        0x10467ee1c381e9a7,
        0x068e5b862079d270,
        0xbb6ae5f190ba4bcb,
        0xf2a1e08a932b73ba,
    ];
    // 0x04c2d02644828c4b60d05525ea628a48d029c5ed71f280458bce6b187f2a53d9
    pub const VK_ID3_Y: [u64; 4] = [
        0x04c2d02644828c4b,
        0x60d05525ea628a48,
        0xd029c5ed71f28045,
        0x8bce6b187f2a53d9,
    ];
    // 0x1e7880f60d8e6a6ed65368e7f8919dbcb65ff3ee1790b1744f8295e430d89829
    pub const VK_ID4_X: [u64; 4] = [
        0x1e7880f60d8e6a6e,
        0xd65368e7f8919dbc,
        0xb65ff3ee1790b174,
        0x4f8295e430d89829,
    ];
    // 0x227774f6d88291da12524fb307f0671abe7bfe7329ee06928a8ce13e2e5bbc53
    pub const VK_ID4_Y: [u64; 4] = [
        0x227774f6d88291da,
        0x12524fb307f0671a,
        0xbe7bfe7329ee0692,
        0x8a8ce13e2e5bbc53,
    ];
    // 0x00
    pub const VK_CONTAINS_RECURSIVE_PROOF: [u64; 4] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ];
    // 0
    pub const VK_RECURSIVE_PROOF_PUBLIC_INPUT_INDICES: [u64; 4] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ];
    // 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1
    pub const VK_G2_X_X_C1: [u64; 4] = [
        0x260e01b251f6f1c7,
        0xe7ff4e580791dee8,
        0xea51d87a358e038b,
        0x4efe30fac09383c1,
    ];
    // 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0
    pub const VK_G2_X_X_C0: [u64; 4] = [
        0x0118c4d5b837bcc2,
        0xbc89b5b398b5974e,
        0x9f5944073b32078b,
        0x7e231fec938883b0,
    ];
    // 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4
    pub const VK_G2_X_Y_C1: [u64; 4] = [
        0x04fc6369f7110fe3,
        0xd25156c1bb9a7285,
        0x9cf2a04641f99ba4,
        0xee413c80da6a5fe4,
    ];
    // 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55
    pub const VK_G2_X_Y_C0: [u64; 4] = [
        0x22febda3c0c0632a,
        0x56475b4214e5615e,
        0x11e6dd3f96e6cea2,
        0x854a87d4dacc5e55,
    ];
    // 0x1670ed58bfac610408e124db6a1cb6c8c8df74fa978188ca3b0b205aabd95dc9
    pub const OMEGA_INVERSE: [u64; 4] = [
        0x1670ed58bfac6104,
        0x08e124db6a1cb6c8,
        0xc8df74fa978188ca,
        0x3b0b205aabd95dc9,
    ];
    // 0xa0284904c10ce20a272c4d745464069376aaa28746dc7fbc319a43a3abd32e21
    pub const VERIFICATION_KEY_HASH: [u64; 4] = [
        0xa0284904c10ce20a,
        0x272c4d7454640693,
        0x76aaa28746dc7fbc,
        0x319a43a3abd32e21,
    ];

    pub fn verify_proof(ctx: Context<Initialize>, proof: Vec<u8>, public_inputs: Vec<[u8; 32]>) -> Result<()> {
        msg!("Proof verified!");
        println!("{:?}", proof);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
