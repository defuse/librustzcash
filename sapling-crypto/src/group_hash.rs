use jubjub::{
    JubjubEngine,
    PrimeOrder,
    edwards
};

use ff::{
    PrimeField
};

use blake2_rfc::blake2s::Blake2s;
use constants;

/// Produces a random point in the Jubjub curve.
/// The point is guaranteed to be prime order
/// and not the identity.
pub fn group_hash<E: JubjubEngine>(
    tag: &[u8],
    personalization: &[u8],
    params: &E::Params
) -> Option<edwards::Point<E, PrimeOrder>>
{
    assert_eq!(personalization.len(), 8);

    // Check to see that scalar field is 255 bits
    assert!(E::Fr::NUM_BITS == 255);

    let mut h = Blake2s::with_params(32, &[], &[], personalization);
    h.update(constants::GH_FIRST_BLOCK);
    h.update(tag);
    let h = h.finalize().as_ref().to_vec();
    assert!(h.len() == 32);

    match edwards::Point::<E, _>::read(&h[..], params) {
        Ok(p) => {
            let p = p.mul_by_cofactor(params);

            if p != edwards::Point::zero() {
                Some(p)
            } else {
                None
            }
        },
        Err(_) => None
    }
}

#[cfg(test)]
mod test {
    use pairing::bls12_381::{
        Bls12,
        Fr,
    };

    use pairing::{
        Field,
        PrimeField,
        PrimeFieldRepr,
        SqrtField,
        LegendreSymbol
    };


    use jubjub::{
        PrimeOrder,
        JubjubBls12,
        edwards,
        JubjubEngine
    };

    use group_hash::group_hash;

    #[test]
    fn test_group_hash() {
        fn find_group_hash<E: JubjubEngine>(
                m: &[u8],
                personalization: &[u8; 8],
                params: &E::Params
        ) -> edwards::Point<E, PrimeOrder>
        {
            let mut tag = m.to_vec();
            let i = tag.len();
            tag.push(0u8);


            loop {
                let gh = group_hash(
                    &tag,
                    personalization,
                    params
                );

                // We don't want to overflow and start reusing generators
                assert!(tag[i] != u8::max_value());
                tag[i] += 1;

                if let Some(gh) = gh {
                    break gh;
                }
            }
        }
        let params = JubjubBls12::new();

        let expected_points: Vec<edwards::Point<Bls12, PrimeOrder>> = vec![];
        let domains = vec![
            hex!("0b4c693149060f99"),
            hex!("982fc029ed2213e4"),
            hex!("5a01101e28c6b466"),
            hex!("a3716d31a988b6e6")
        ];
        let msgs = vec![
            hex!("076cb41a3d40719b258b9f2d959e6e26555950ba05cee053cda7fd1b7a3e94907906959d36e3462ab8c8ec070fa99dbc0bc6e5554e9a7c30ba875fa3b3269e0dbd30b63b19313dbe3a50279e7d5d4e3787ebc794bac60c683ca8e3220fcf8a2bccf061a02635b32a1e368e82b345ae5a1af5150b0c703892b8da35a070ddacf97faf529602ef4071908b7df09d66c55da5543d1669888d64ad17785e555d202e00a8005afb0d490c0807377d45227f4474e1aa527694b0b7a0b17608b09bd44c8d8aa9debfe7a7620cd60fc623bfe26ddfb91e9ae32138dc3a0585c42ca89a61c299f9fe79f0e0237d8ee8b4d30186b77a88bd6f0d20b753a496c3bf399e9ca1b62ade8e7dd662c5a19123f077066be5dfa90e5a06c2624046ddf11ce7e53cc3f88d8aa2834363c8d38dc612d58a980317448ef79ccebdcbcd9f03d31a13727e7bff883135b87072d164436b1da4b4f214601cef55a5c5721618c0c6dac59464084b9ef9620729e8f9c7ef2c2c806f2506ccf7b8f2edcb675f8be1c50b288c3653e8db78baa785f0a0c2e059fdd2cf0104b597cdb9b82d03c63c161085da637fbfe444d3e5f9b9143088f2be54c1d293ac03967299dafca5b3c4f5e3257fe9010325531c9cc1cd67b70afad697ad750d8488b977ad3d3882f2d0050a0e22aab95eb13e80ffac09003a4282ef27820636b17ed11bc29bfe9b1941a0d5534510483f4178b327b7297b95b98832313999bedd43018e27ffb9d69777a4d7aeb1e5702171317949cb2577b897b059a976ce944a750b77ab07cf6ed4c8f45559f46dedcea4d5082772ac11ade8d1a057ce584000ef0bf4aa0b5589c5cca54fb69cc67d486ac040480efe6e5a38b3cb82c2b78ad125ee6cee4261644f38bdc34bf4da4dc087a394c313ec839a8a4ba6bcbf1eeb4afc15fd46fb9900a4744943efefffc477f8ca897e683d7d9f007f6c73ca4b64eb4ffb253f0a6137fc2956c74b9e869527b3bc5819c3d6cf6523bcd7a44599fdd6baeca754bc55d9ff338e64399d6dcaf50df0f1c5873991eb83fb92d3ce9b2bdd133b24b9edae467371563167c9402adadbfeed5d3f571a9fb1d5090004213fb333484d15ed753fbaeacf16aeb9764a960c5b7b382d2d925050fd103a0bc1ae709cfff11e701cad99d82eb42a493ef2f32862ac07cbe2494b354511cdfc6417dfa647c967026552e39cb96282e40c4ad7c3d838b2edb0df0e9b98bb90280998ff8cafa456fafaa6a769ad47599bec3fe242211424369acca65ad5507f96881130262b407d0f5fdabfbdb1b97570dd3b397fce15234674a90812f7eb6ddc5cb21db5b64e25a8449e67a7ae09d2e438d1da8ff5e06ff417d5c0d432a2fe811ad216ccd46f71202b0bd045dc3664f8d2a4b78de8991191876c38959056c02764d1cabec14de12aa059dea10d0d20b75a7f"),
            hex!("814d0b90d16d7d4c270913ab375291651ea130aa5608d5fc625a8daaf49d15ac3977f879126f19f651b8b167f5debf3e824efcf7fe27789025026e91db64f271ea2b3787448bddac9992d45b3419209bd4ec43dda893af5ce21939c3cfceef2b2b3bbb8bebe19cc3e5068c54ca4eb0c80152f0a9738f01a48f2b86503213a0ea50008d77a96842749bd3aa09368f641df231b8ea58a1ab76b311cc3c5f6341216407f0e3bb9e20efc50b8430ce3eb848c53441e1c22cd358042d4f7fc0bcb41a3b45edd30d52b0acb67ec67312fbbf47b3fa1716ab14caeab013c30a886070b9e8bce8e3c6d24e57e3e96e9ad044a7e2a6ea13a6a2dbffca1b81ba76075831b167ff423523429ffa42a03654329d86475642d1b1ed9f597d45147884a88844d33bec515a0b38d1e0351647c7c522bcdcf5c9286afda49431c2e156f4a8859d28b6e19eaabee2adf42acadfd9ae8ed6b1031c4d4c9feea5380db5e56ec036ff656c3eb59f11e58bffbbd988177bd2a0116ca2870e4d471f5c7f5c36463089118ed57073bc069eb209264640b3048a0204b4af3d714e64b502b4168514f86d0269346e78ea9d0d33926e3c784f3ea3dd07ad1bc188fad41880333786e821322d0c8be675fde29ea0a3f6ed30fc46c79447f4368ffc8268fba099b5b0ca7cb75995313a556c7ab339f0c03af811bae47f4806f2b6eacec66f15e4656cefe3df08887a625f14bf497d2f8bf99a68e7361193116a3bb1765d7e8e7bc4d4864c88dbfb818334b8cfb5dca3620e097e40756a1f7d28c7868d7dd65e49bc5155e26c8973d476e25fb1ea75e382f654bdfbe2bc45540f215a357fa29cd6238e8bd1085ce625f43f40eaa1d20bcf9772890725ca27b4d38c9870d91530f95a406d90b1b77c700d0d0296a1ded208507b7284fb4d2202c7fe9a20ef96700f7eb70e3eaad51bb95e215da42794fd17d0aae260cecccf99dcbde251f6c115acbee8a30745bffa43f572383112f45e97a3316edc96a8f6f9dfd55cc99f878e627a46ef65ac95f81eedb14106dcb0b2c6c53bb9034ead2338b28b20c09f2f17f202a7b6c1be3e8885f119e4f6599e05ddcd5b74a99af7c935228899de177a93438da57ce396782f65032ef40805ec01e7375b07dd71929fddb88995da03a879437f34534514d48b1ae5ba86e096d00951e7b96ad890ed85dd4a2f79214f7193ca02facb19d9d99ca1bca3ffd237abfd57c4d9b210115f5d62a092d36b100b1f84b17889bb59fd84d47e046ec646ee99a3c14c2ad9fd78628c76035ad126ea3ac0f7543d15f569d9de5467d54f3536105e7f9d0592d2e3a3b9175f252e6243fc1694231ea5159f865d2997eeb50241c4a7ab5e0d97ec760c40212116f3b1f139c48be3696c717329878dacdd7376c81ef8c077d2eac4d8516551cac435077d2359a24e624a1b26b8"),
            hex!("6b94782eb58954430de33992f53f4ee00f99977a8cfa30eda3021673182e39b046594fc8df623e23a58bb136aa294feb4c4e7a692db5b537176eb0c521fde239eb6a5218fc82bcb7de114e05120b91a605beff13a9968564aefc15255bfb6ba2ebe37832ecf9168ac65dd01015df8b9372472c9cc5b4446b73467afdabdd25a629eec87ee0221a93f7bd88125f1178d427cb21c288a8d8c3cb4d1843a57d4637bdb1eeffa1dd8276ac680140e73657ed4a84563f5b09ddfb604bf4c2ddd6075db6b02d78c4cb02a4ce3aac20aa15e3a5d5b30669b343c5239c134f76de18185a6e0a66407c26023d3049adc74b0fdde0a9176cc3e2ccb9bf03ee306953a9ac1312c68c65a284110590c29dc8baff373ad49a40e3de8098c67dca210c26f768745bfa859b0a3fc5e88985d046abcb5e6155e0e23115ad062d10d1b73af0680f9a141e379bbd177bb0f9ea3bb5f01b8a5665bf0a9ea1266138bd688572a779c0b0a69e57f40dfb418d514f8ab058ba28eeb21cefa3634300e89d204c4236848f5dd11130ba13d83c10afebd4f822c26ae0ed7ea3bc3ac92817ae405c3cf886c81bdda07447cebb71a8bc3d26973afa57e0365b8fc8c0658131734ca29afa10eeb0895d3cd5f0b84a625f53f9f3dbfc5bfac233c7c7d3ac8829475076b14e5c57026c4a63bc9a347ec3fdbcc997cea4c273f923e01fa0781f847cea54a81a4ed14117a46b0894bf4816a46b1f7af963382eeb6f58ed35fa70572a7bb3f2bf97d8c549fc55d7bf562c2d8c4d4d748b7d062cb41b025309b5587a4b2a0cedc85280dce2f58dad0943ccd591b2ee7bb39378efa09cbdd975af07c33862f560d03e1621374cadf6cb05484a23a37d3c200bc3e1780ac65b02bab7db25fe89dae218d9e7dfb60473f9f5b21a007b714eef58129518d637bd581922a54629bef62a575a98d513d9abb90b023752439e20d897242fea3fabcf3fb7eae7604fd0dd1bd5653c41276660a15d39093e4c0862d48333b77b8bc3d64a5908614b3af7c815406b3ba21598cf0c70afe7b8a8ca668dfc8376e15a0ba0d9b9a79552993d2a24f3d0689964e7141d4255a19cc9450806d632a73b88a5768d0de18c20f2b00268ce5d3a8ca2231ce3d8924335f650cb98df84bb58073166848786de3d1823075259952389c50bced6c8921e7b019977d58c5a07e57eb21ef907a9dd465962bd52beb0cbad28b434ed7fecde21953b3fd260a90fd9ab7495c47cdaa4e73fb487a5239e03fb1e74e8cf6d691126813a6f5c16ae948da8cda292fd46e79f7551f8553b21249a5679a5f56dd4b08a136ce0fe8146bddc1d60e0a9a757e1a608749bcce4ac57cdc8fafff59860fd22f95f46f3ab50812a6b877f24941c2b8e1724e527ba9ad9331e004792d2b292a7edee983e6cc22445f4fd43e9f006ddea19fbe6269f29cd"),
            hex!("883b1a34c0d0a8d9387cff427ab0b476b4c6f5a09e30be336aa2bc35f38ab4cc398673fae02ab727f069e887006ac3c0b891477c4a837456417c9e817c5ccdaf12b54eb90cc18a9c30673ed36bd7fba45269437c09feff9a1bcc2993533533227a535950c98cc77f00f4e77c34184b45d865942a3d72acce4922e9c284df5589d506404648c5c3bc7f217ca023b364e74d591d75c8a19e063b2b82c28e27ce5b2afede6699d6bf8d4931244aad739b285f410f1e95c6d5e9a34982908d1c7be5f0365dbf30a7d6dbb2821bf6d55c142a15bd561f3eb202da9b59f891584b35011f5cbe77cab38263b5773360d52e29b765a03c420c75ffa2c91e8edd4a9bba4035119bc3a088ec303a6344e4dafbd8d1889cc9b3d56ee08cff1530f1a1d9a77712006ebc9a3d80f5a69e17b0a0f65d97e53e7852ddd95760c68bba78d326a28eb07908c8e630dd6e42ab272e980774c5c51825c02ed8950fc85f5711c80d5fed1d72162a4bcaefbba33c09d09e0611ad5c2bb2949daa55861b5f6f8beea02ed392d09ab0489f89b3875fe7ae90c6ecf0bddb4b9fbce6dc1f97b12ee058f00cc9672c35702dac4afbd6776e1e222d2182358e91e8d4d20b99d5f2adbac7abad48c07afc94c212278812bfc5e76da0e1b97b850020bc1ba3c0bc0093d8acf4a594a8fc1dcea70c613fcc550b7654a08ed9f50a34c21626f2b599a15a097c85785bc93d5c5034edb3590f0c6aee3dcbeac5f15be5bf6f179246c347dce5040115dccff18fbcd9e9ab485d85e89dc0995aaebe4eb8796fb01b03fe5f5c6c903845a0fc02c69d2543684a33e4eefc7e0d5635192c82fdde5e7487df95595a442a6a71e98e09073cdf318f808fd916e0de7e9182855023f4844042764fef897ccb500d520651a262d27f46c16e38187da84a3d7f9c42bb65e1940a1f3b3c2eced613e141839d77851ef833f94d9a5ec073e1b12d0a2b59b8b90c5e560baceacebe569f8ded41a67d15cd7486f45aacc1fa4e50805485a698d2722ba8fbf87442e1a2e46cde49f1600f97ae3d6f67169141a40b0628c9994c115377db6c30d822381dede366f355c71ff8c38ae4f60da77bf40e3da97f77ba6756ae766a17896ead15d4367f43fd2a4c096feb3e23d4a061fed2422e9e424f1e81e890ecc9474bb5b7d8231447d5359efa8ce4d40698d329cd5d069f50050d26a0c613a4cc3d70774537af278643799bf5d336d71a19b83a5660ebc6fefa4664167c497478ec4e8b0954fc1a41948a84f5271c4c2ab93c6848fe5abd28fa68b47ff79ba1475a46d2ddbf5a07600ee4cf76bf85bccc53d5b5094f6728542951a63c71b251429e8bb4fc95cfbf283db2b0ad2d16bc2d8ba07b193249bc974eb5392b7e9c5a0e1056ef0208e6494abe61f37d64b543acf509926f7b9f1724ce39e7690bae4d918be3295fd2")
        ];

        let ys = vec![
            "8502599294297669157183582041043506286304348771153601905088214968423735432772",
            "33965310400650966081486833884535323100804531882948083108992748314044766607474",
            "15277426621450144245366093477629790944965634885834431068514786570163432982421",
            "35526445498940553839675656924597924255939683458731864358252626115877434851278"
        ];
        let xs = vec![
            "23479585783156774250942425515624703792585157520679515316930097097463607664576",
            "30414851484511157010605445406157992259368652076831836832380699127755424334026",
            "34566775937206506013251574661622220967552701387632591444790779184716709173668",
            "32625571922270028001313966220069858825087579007581150636305043327525524456655"
        ];
        for i in 0..domains.len() {
            let domain = domains[i];
            let msg =  msgs[i];


            let gh : edwards::Point<Bls12, _> = find_group_hash(
                    &msg,
                    &domain,
                    &params
                );

            let p_sign_false = edwards::Point::<Bls12, _>::get_for_y(Fr::from_str(ys[i]).unwrap(), false, &params).unwrap();
            let p_sign_true = edwards::Point::<Bls12, _>::get_for_y(Fr::from_str(ys[i]).unwrap(), true, &params).unwrap();
            let is_one_of_xs =  p_sign_false.into_xy().0 == Fr::from_str(xs[i]).unwrap() || p_sign_true.into_xy().0 == Fr::from_str(xs[i]).unwrap();
            let is_y =p_sign_false.into_xy().1 == Fr::from_str(ys[i]).unwrap() && p_sign_true.into_xy().1 == Fr::from_str(ys[i]).unwrap();
            assert!(is_one_of_xs && is_y);
        }

        for m in 0..5 {
            use byteorder::{WriteBytesExt, LittleEndian};
            let mut segment_number = [0u8; 4];
            (&mut segment_number[0..4]).write_u32::<LittleEndian>(m).unwrap();
            let p: edwards::Point<Bls12, _> = find_group_hash(
                &segment_number,
                b"Zcash_PH",
                &params
            );
        }

    }
}
