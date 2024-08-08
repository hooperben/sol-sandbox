def transform_hex_string(hex_string):
    # Remove the '0x' prefix and pad with leading zeros to ensure it's 64 characters long
    hex_str = hex_string[2:].rjust(64, '0')

    # Convert the string into a list of bytes
    byte_list = [f'0x{hex_str[i:i+2]}' for i in range(0, len(hex_str), 2)]

    # Format the byte list into the desired string
    formatted_bytes = ',\n'.join(['    ' + ', '.join(byte_list[i:i+8]) for i in range(0, len(byte_list), 8)])
    
    formatted_string = f'[\n{formatted_bytes}\n]'
    return formatted_string

def hex_to_u64_array(hex_str):
    # Remove the '0x' prefix
    hex_str = hex_str[2:] if hex_str.startswith('0x') else hex_str
    
    # Ensure the string is 64 characters long (256 bits)
    hex_str = hex_str.rjust(64, '0')
    
    # Split the string into chunks of 16 characters (64 bits each)
    u64_chunks = [hex_str[i:i+16] for i in range(0, len(hex_str), 16)]
    
    # Convert each chunk to a hexadecimal integer
    u64_array = [f'0x{chunk}' for chunk in u64_chunks]
    
    return u64_array

def transform_input(input_array):
    transformed_strings = []

    for item in input_array:
        memory_location = item.split(', ')[1].split(')')[0]
        memory_value = item.split(', ')[2].split(')')[0]


        name = item.split(' // ')[1].replace('.', '_').upper()

        four64s = hex_to_u64_array(memory_value)
        # print(f'pub const {name}: u128 = {memory_value};')
        print(f'// {memory_value}')
        print(f'pub const {name}: [u64; 4] = [')
        for item in four64s:
          print(f'  {item},')
        print('];')

        # formatted_memory_value = transform_hex_string(memory_value)

        # print(f'(\n    \n    {formatted_memory_value},\n    {memory_location}\n    \n)')
        # print(memory_location)

    
    return '\n'.join(transformed_strings)


inputs = [
  'mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000002000) // vk.circuit_size',
  'mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000003) // vk.num_inputs',
  'mstore(add(_vk, 0x40), 0x006fab49b869ae62001deac878b2667bd31bf3e28e3a2d764aa49b8d9bbdd310) // vk.work_root',
  'mstore(add(_vk, 0x60), 0x3062cb506d9a969cb702833453cd4c52654aa6a93775a2c5bf57d68443608001) // vk.domain_inverse',
  'mstore(add(_vk, 0x80), 0x28757cafd515f607cbeaeb4515495c3b3c8713bc62bd31b1864c27a40d2535ed) // vk.Q1.x',
  'mstore(add(_vk, 0xa0), 0x0c672512cacb2b2c73c1c5b5c9988e9f6b34abe07690abcd26ec13748eda964a) // vk.Q1.y',
  'mstore(add(_vk, 0xc0), 0x2ef390db388bde18cb22e5dfcb283367b1f35ab86f9fd4b1a184b5cfe346254c) // vk.Q2.x',
  'mstore(add(_vk, 0xe0), 0x0fb2d61440a13ece99bddada11647155217bb6f044b7e97bdf22ab2743c39201) // vk.Q2.y',
  'mstore(add(_vk, 0x100), 0x15b545b5aafcdfa7b753a91e0ce9b8c1b2175899a46b298f489344b21ba94ef3) // vk.Q3.x',
  'mstore(add(_vk, 0x120), 0x0fd2e5dcb2e98b92c173705cfe2df0dc40b9c7ad09210695a2f98515072a2894) // vk.Q3.y',
  'mstore(add(_vk, 0x140), 0x23d2dd0b829610c59d099018508ee128bbc71ab68002411490e04a9122ece9bd) // vk.Q4.x',
  'mstore(add(_vk, 0x160), 0x2f11872338be4f110399909a8cde9b32b94003facd05d98e241908059c2c20c7) // vk.Q4.y',
  'mstore(add(_vk, 0x180), 0x186ccce2aa7fd77041be861cb5ca726a9eec91d2b6edb4755370e23b665b86ba) // vk.Q_M.x',
  'mstore(add(_vk, 0x1a0), 0x0c6a6f7f5826d7f690605d9228c4b42589c3b2970c875452526bbead94525392) // vk.Q_M.y',
  'mstore(add(_vk, 0x1c0), 0x0ae2edab13a0ab00a9378ae4863d6a7e4da5cbb5d54edc032081840be8732115) // vk.Q_C.x',
  'mstore(add(_vk, 0x1e0), 0x1ce293e6e839821a81e751636abc7d8fb59b564c12aa99929158ace8234ef1b3) // vk.Q_C.y',
  'mstore(add(_vk, 0x200), 0x2cb5a3fc3d66ae99856b96b5048e69b57ebfdd0e97bd573c59fb1e2ab1c0b6a0) // vk.Q_ARITHMETIC.x',
  'mstore(add(_vk, 0x220), 0x2f8c572aa95215ca5fc74d104288d19987f8e650ab1e69897fd1473c4755e854) // vk.Q_ARITHMETIC.y',
  'mstore(add(_vk, 0x240), 0x1ff5f913ca607ba54b4ace25cf8647967e46a355ba1a069304939a326b878bf6) // vk.QSORT.x',
  'mstore(add(_vk, 0x260), 0x19c2d7fbdc0b9b88780055df45f0184f2c123ca016a32adc6f09f286c9947d0c) // vk.QSORT.y',
  'mstore(add(_vk, 0x280), 0x28f715c93bc880fbc43a5f2cced51a040c881a59d47e2317cde990e3d97e458a) // vk.Q_ELLIPTIC.x',
  'mstore(add(_vk, 0x2a0), 0x14a796f0bc75f4e40a23ecd4af7de1ca745879d983d68d056469b8d20705275c) // vk.Q_ELLIPTIC.y',
  'mstore(add(_vk, 0x2c0), 0x031fb161c3fbe55d75807bac763a4ba43d14a402f896de249bb48174a812bb61) // vk.Q_AUX.x',
  'mstore(add(_vk, 0x2e0), 0x1e22b35319ce5346023c24d3f892fd07d34b02953961f01696417deb221c3509) // vk.Q_AUX.y',
  'mstore(add(_vk, 0x300), 0x22f8fcfaa09563966966b7b23b392ecbe951696a36ead6ff31ccc52b32f97866) // vk.SIGMA1.x',
  'mstore(add(_vk, 0x320), 0x1872df144cb2c2d16165380b232e9c555bf67924b48c22b53ca0c631696785ad) // vk.SIGMA1.y',
  'mstore(add(_vk, 0x340), 0x1692f386300cfe92ee01f9781fcbec9438a46028841b206af0a30dc24ae3d5ba) // vk.SIGMA2.x',
  'mstore(add(_vk, 0x360), 0x28b429cd57b4d6928aea17dc8d8c0edb18d614ff3ac51b6fa8f6fa20c5d265f9) // vk.SIGMA2.y',
  'mstore(add(_vk, 0x380), 0x12496a2cda99445e7767c5e810f2a7f4318e3e7be1198eb501ac5da763f0dbc4) // vk.SIGMA3.x',
  'mstore(add(_vk, 0x3a0), 0x2b2a4775c72353060dd186b99d35266c9f4d0752700d47ca86439d496c750685) // vk.SIGMA3.y',
  'mstore(add(_vk, 0x3c0), 0x23b4d8c19644459cbf9a3b61632d76d7861dbe78f11a7f8c0fc09b63ea4258f7) // vk.SIGMA4.x',
  'mstore(add(_vk, 0x3e0), 0x009cfb99f579cbddc0c8465bd277f9295dd694ab201215c3e96d7b15a2951981) // vk.SIGMA4.y',
  'mstore(add(_vk, 0x400), 0x03c3f013d7bd74d9d03aecd380c7bcfded7bffcac5e9ca72f4fece642d94840a) // vk.TABLE1.x',
  'mstore(add(_vk, 0x420), 0x00af81192977a3213784968885b8c6e575a39c067134bc6da87d45eacf108da0) // vk.TABLE1.y',
  'mstore(add(_vk, 0x440), 0x2efcd1e3d0e83fe2adebd9bed2dbea769748c27cbe1c3b1318eb029ff9fbae25) // vk.TABLE2.x',
  'mstore(add(_vk, 0x460), 0x03c3d6fb451f07a91ca5c7cd137769a836a0e3627521042cb0231aa62cd5201e) // vk.TABLE2.y',
  'mstore(add(_vk, 0x480), 0x1e94cb1d304dc0752db812d57390ffa30a6e4c0f924346e0ec56bb4baca8eb0c) // vk.TABLE3.x',
  'mstore(add(_vk, 0x4a0), 0x0a2f8564302b49ffbb94512c138be76da6e58c12e7f57b51fdb35f9e6c4c1fbc) // vk.TABLE3.y',
  'mstore(add(_vk, 0x4c0), 0x0d141201ddcd88cd2a0ada7bf51517c3c08b8dbaf05d09b7119faacc3706e1ae) // vk.TABLE4.x',
  'mstore(add(_vk, 0x4e0), 0x2a6d896384beaf8a2b86d8438e1b1e32f5f21b714dfdb8536a22ab3c0442fe12) // vk.TABLE4.y',
  'mstore(add(_vk, 0x500), 0x1aa5249a55d63edf065fc7fee5f604006244319b695612028eb7bcb9350f2e37) // vk.TABLE_TYPE.x',
  'mstore(add(_vk, 0x520), 0x2465961b676caff24e134a16641123d182dc02bc8390aa5ef6e0fa77566f9d29) // vk.TABLE_TYPE.y',
  'mstore(add(_vk, 0x540), 0x09cc94458ee0afa1009fb97bb061b102d970106e6d64aa00df4d555b6136f805) // vk.ID1.x',
  'mstore(add(_vk, 0x560), 0x10ac975fabe85d5531e9a6abd70d039a412ac9582e7f7957836998b801af25bc) // vk.ID1.y',
  'mstore(add(_vk, 0x580), 0x2919e193e97154c7400631f427e56fd630fa438b112cff9b4fcd146565db34d6) // vk.ID2.x',
  'mstore(add(_vk, 0x5a0), 0x00f8d5e9e9c8137e87911a8d8aabb84f21d0921cb24bdd646bcd6d731d758d3c) // vk.ID2.y',
  'mstore(add(_vk, 0x5c0), 0x10467ee1c381e9a7068e5b862079d270bb6ae5f190ba4bcbf2a1e08a932b73ba) // vk.ID3.x',
  'mstore(add(_vk, 0x5e0), 0x04c2d02644828c4b60d05525ea628a48d029c5ed71f280458bce6b187f2a53d9) // vk.ID3.y',
  'mstore(add(_vk, 0x600), 0x1e7880f60d8e6a6ed65368e7f8919dbcb65ff3ee1790b1744f8295e430d89829) // vk.ID4.x',
  'mstore(add(_vk, 0x620), 0x227774f6d88291da12524fb307f0671abe7bfe7329ee06928a8ce13e2e5bbc53) // vk.ID4.y',
  'mstore(add(_vk, 0x640), 0x00) // vk.contains_recursive_proof',
  'mstore(add(_vk, 0x660), 0) // vk.recursive_proof_public_input_indices',
  'mstore(add(_vk, 0x680), 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1) // vk.g2_x.X.c1',
  'mstore(add(_vk, 0x6a0), 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0) // vk.g2_x.X.c0',
  'mstore(add(_vk, 0x6c0), 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4) // vk.g2_x.Y.c1',
  'mstore(add(_vk, 0x6e0), 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55) // vk.g2_x.Y.c0',
  # 'mstore(_omegaInverseLoc, 0x1670ed58bfac610408e124db6a1cb6c8c8df74fa978188ca3b0b205aabd95dc9) // vk.work_root_inverse',
]

output = transform_input(inputs)
print(output)
