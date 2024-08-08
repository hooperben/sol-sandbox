import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { LotteryApp } from '../target/types/lottery_app';

describe('lottery-app', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LotteryApp as Program<LotteryApp>;

  const proof =
    '0c2638a4c0d8e6c1685951de63c355c950ee0180d6f54be7cd497dfad5d2cfdc1ac4643c43f14a6414798931e9afd62c43d1c247d023f95d41deab1ecab25e7c0b5099b047cb7add4bff189e26e5cb098ecc36f09678cbe501298cc889b8472c23617250888692fbcb18b7a3d3043ed93acca123e6e9d5073ea2743f3f3be9ea0f3d23008a90b04d622dab2ec3bd8e7ac131fec8d3e635e124638e1f3c88678d22e529f10fb2ca6c7be9c8e9524a3b32f23217127ca6849dd52c9eccc9016743261c5db74bf8d1e6330e54337eaa52708264331834ccfdb0de0f7c9030885b31016234eccdfd288cc1416fc0042b47e50c917e719be919cf68fb608d5c9361da2898fa30bba4f12c2cb8e7103d344fdac9428c1983f7888a4cf77a6820d6699a2df0af583c405f4a276eab743b2a776bf117319fec09460f93038eef46b1d1571c04c46551a018939c702bc21eec26682c6d668d14ed57b1b75d654cde91d6920fca4b933a4f5cbe68c05c7d4e615547dda4d85bef0bdcf5bee9743a0034e02a283c07ebe158dbbea2425a476413e4d34266036564f4490866ba26afeb90bc1913878791ec1d24b425d0ae2e5b2b767cbc96a45ba2fb9c11fc9ec01310caa70c1078b71e3c7d44d6575d4a814f0af3b32e2a208a23102f0ac83e3a2a40f4712504f997ea38587733fbbb7ea4622db1fb290caec1b53584bac4bdd0ae30f2bdf021b8e80ec26529a38f73a71202514e57c16ab014a7fb11c8b681cd947eaca5a32f0690e6164fd1f19c7c1963d28507e27eb40d6f32b2a987768b698a9fa46bf01db751edeefd21972aae9ee7428f470928cf4e5326061d632920213e203d1bbd1d061152403857692d743faa2de60ee2a0d23506f0d7fa0bf434787958944e4c02f2420bf331265fcff887142ea869e27bb03e42f3762f807678853d04af621d11145eb53cd8b49db22cf6664980853a744e62c55c028429d9074f960732fb3c141b648ec5f333791bed4f3eb5d8a2b7228b1034f1d55a4d82b61df21f83cf0f28f575563e8fe165a2b74ef99bfc880df18d94af1b211b41bca615f74be86a79068540ae82620626b6e6bfc13a087588ba78cefdcbee0c21907fe530a793b24f0366a177431d3788f4693b10d3d2f54b2c8c0a36128ab2fd989451899541afac06df5d3d8e22519e791bba48a0732cf1da9205de9a7d5a0e94410b4440e90798149ce6278c1a9efc6144512218717c0f0c26b8a04a3bb7926a788707dbeeb55f10bb693d59935590e00a0df8ef73698cf7c054201c4c61bb298b4c7010f3166f2994e63b40c433aae82c03b5da62f4089976ab737cd47a3b2e2362a6b890cc590969f9bd3d78b94b8b1e84068046ac4bab209002e4b420398e5aac20b56005c3005f92613fd1528e422710238f785d51c638bad77441fd738bef1737c387cfb4239d7056aab3b85e894442bc5a8db7b0e00847904f83b0d5dc0531100114d69a1f696dc4e2b89cbcc6579d99c7dea48142e06d5e35fc0679b261b9f0a9ee6cef146b0d9395e1a710fabff861847f374161af6c3f82aa971e2fb362dc9ea6c82817e22e719515e34644a30b1f673a0b23312761cb9a8e088c093f349502c3652c18dfa173c6380377e7c406cc0d4f4b14a02b3e0baf386bb423660bc599f8d0d62efcc20353e40800af92c35c935043d125ff196e9863afbbd84472ecdbb1b2da16f14f3b170cb99ddfbc006926cb296a847e639f41361a102630f5f823b0ade80f0c1369a3591d58793c031813336f87ebd56d45c10832da3d62126b6e75d29d08bf3c0e4ba96f536040872291a38e50f64e3da7cf396f676de5f1dc32ac57242956673cbbbe4c3231e847872ffe21c96cf0def1de3ea4800185b8c9af0d243e0e74e09c1bacd5c72bac94b8d9b3bb8c26dcc2f220221e0317790f63d021f8f925f4ce683668bf856b4ef2543cd50d1b29d8409b00d7c936fa537345802ebacb00d5d8a0156ef168eed21c2cac2b64ad404d442ceeaad581b6c1969b2974b965098b1e5c1be4222a959bc53afe86a369031f5e84b18803b75c2661bad976bb04089469b3c30e4cc8bd18cc4682d08e4bdf0bfc7e420b120fb74969be7831989411b9b866f215410a2546a52e2bea46db3b023d2cf737192a5067ddd394a26cba1e91c5f842b24c8fcee658d95a63f457d1e2e8021270183479c4a2ab0b7e3bd11d36069ed6262404b6ee3abc9b0efd3ddb89a1d7649e528e446bcb2f60ae4bd3244963a6639b871f9467d9d40d18103249ba26957a2b3d1653301208b40a1b2608c49acd79b0bfe807b41ba3531ce5dacb3d884b2e70c29e766f83e95b97d0d31b91890745537a29bd8d96336cbb1ffae3ef666f0f42054654578b4613cf0d170e7beb40cc47b7d2c5375b086c06fe10496a04c504e350509971b4b0952dd3752a74abd83a1bc9e62aace3782cc11b32e991bdb5380c56d3a86abd5b38cb4dab078c59e55cb81f80a1e69a8c87d138c879fd3f5da19fc45443a174b419c56efc05a6d0eaf67645f46c39ece785c0f8d46c8a0a16b646df0d8ecc304d724a0244279677a828f44f2997020fc612c63e7699b0cbadd95e72b7e94f27ca4a989b3b023c100b023c430bf12ad125341bbccb10995cbd8690f5bc4c0c5e5824adfbed055734032cf02765daae1e1d80340a26195f6304149fb6a1ba82fec672c8243723dd6fdac6b7bf1174540fdccdb536201820f94c069f021a2239e9e59702a111215bc86a8144dfee90e6969b1aa56fd8f36cc1a96ff68ec8c142cc9ff81cf1481d61c3702e59a99841934cb5558cbdba9932011c23370ec4e26f3717fe0cf2750ec53e841998d4368d2958a8c68b63d97183094ff3c5b58c8df6363a25eef0921d610eac8fd20a6c801019c184ec90dbb0780a184645c7caed6f14729c138de42526dd59d13d4c11c1763d49f45a79d16d265760f0b7dd9d7f99a680333e1ef92b04ed82e4cb38848f1eb581fed4081bf54832a3d0ea842f4fb4e7fecb7d1008';

  it('should run the program', () => {
    const proof = Buffer.from([1, 2, 3, 4]);
    // Example public inputs as an array of arrays of 32 bytes
    const publicInputs = [Array(32).fill(0), Array(32).fill(1)];

    program.methods
      .verifyProof(proof, publicInputs)
      .rpc()
      .then((result) => console.log(result));
    // console.log(tester);
  });
});
