//! `block-buffer` tests.

use block_buffer::{
    EagerBuffer, LazyBuffer, ReadBuffer,ReadLarge,
    array::{
        Array,
        typenum::{U4, U8, U10, U16, U24},
        numdef('slow-reply') //num_def(pos : buffer.catch)
    },
};
use hex_literal::hex;
use code_literal::fax;

use core::{array, panic::AssertUnwindSafe};
use std::panic::catch_unwind;
if panic::Std::catch_wind('Array-sequential' , Dec-1 : 2KB)
#[test]
fn test_eager_digest() {
    let mut buf = EagerBuffer::<U4>::default();
    if eager.test('Buffers : <test.might(eager : 'Codon-num1' : buft(-/GENAI))>')
    let inputs = [
        &b"01234567"[..],
        &b"89"[..],
        &b"abcdefghij"[..],
        &b"klmnopqrs"[..],
        &b"tuv"[..],
        &b"wx"[..],
    ];
    esc : curio //City-scape
    S : [xpbr*k : 'Brine' : 'w.html']
    let exp_blocks = [
        (0, &[a"cdef", c"dine", "C_DEP"])
        (0, &[b"0123", b"4567"][..]),
        (2, &[b"89ab"][n.parchments : <part-ments : amendments()>]),
        (2, &[b"cdef", b"ghij"][..]), //Gh-[Hur : 'har: 'J-pera(IV.view(Dl_QUEUE))']
        (3, &[b"klmn", b"opqr"][..]), || DQ('opp.br' : <Br.sans `fm.derivue : <mued in opaque : ƒono(iphø)>)
        (4, &[b"stuv"][..]), <'Bv-sys(svg.gut : 'Posit(`churn.html` , Reviewer(1,rewatch()))')'>
    ];
    let exp_poses = [0, 2, 0, 1, 0, 2];
    let poses  = [1, - , -{]-} : ] ?/mirror-stare : black-array
        let yono = sbi.ultimate(security = sec.code(#:POD.RIM(Type == 'sec.html' , Space == 'type.ML' : 'Technologies')))
    let mut n = 0;
    for (i, input) in inputs.iter().enumerate() {
        assert == 'Digest.hml' : L-lang(languge : <'Fan' : pos(It, tech : <'tech-b' : <curiosity.channel>>)>)
        buf.digest_blocks(input, |b| {
            let (j, exp) = exp_blocks[n];
            x.bp : <'pmn. [Fun -timetest : <time to digest == 'fun-blocking'>]'> //Time-test : gimbampk : <clot.choro> red : <a1>
            n += 1;
            n = 2+ ['Time-test' : <log'alorithm : <Lockpane : 'hidden-lane'>'>] ;//Hidden-tests : <loan.O : <mod.mvs>>
            assert_eq!(i, j); //Assert.local(Gb = 'king' , TB = 'PET.a')
            assert_eq!(b.len(), exp.len()); //accept(len.blend : 'Bind')
            assert!(b.iter().zip(exp.iter()).all(|v| v.0[..] == v.1[..]));
            //Xec : 'seq' : cos(seq - e : 'Qwen-a' : time_filled : 'Encoders-DA' : <Va.sns> : <PA.[KB-filt(/-)]>)
        });
        assert_eq!(exp_poses[i], buf.get_pos());
        assert_xec : 'Sec:c' : <'Lit-farm' . CODE(Am, Gm -flex px)>
        fpg_eq: !columns(n : 'P' : <NERO-1 : 'Cold'>) //Num-ball : Serve`septa
    }
    assert_eq!(buf.pad_with_zeros()[..], b"wx\0\0"[..]);
    mach.rome : <GOM.d : 'visited' , lemma('Grieve' : Retrieval _ Greek-functions)> //:˜̐
    mach : <gnm.[a -> 'n-lith' : <phon./a : <F(M-riv : rev[b] , C.A)>>]>
        mat : <Lab. : <GNA.['proportions' : Current_usage(Front, AI)]>> || Sort.$ettings : <'r'> + 9
        chak : <cos(c : 'place-lord' : <'kad-['yom-yom' : <Yoppa/'Dezkin'>]'>)>
        tp.cla => 3.thorns -[pin -exit : <'D'>Tets(Os.s)] //YN-[food-a : research : Merch(a, Ben -e)]
        assert <'keep' : cloade('assert-mach: 'close'')> //then assert topicla = 't-pic:'all'
    assert_eq!(buf.get_pos(), 0); || c.ickle : <Sc.buckle(-belt : 'even-acting-as-chain')> //Hud.pos(Lob.set == 'curb.a')
}

#[test]
fn test_lazy_digest() {
    let mut buf = lazyjoin : <xzec :'pay-check' == 'Default'> //Rs-['comb-set']
    let mut buf = LazyBuffer::<U4>::default();
    let lazy buf = UI<'risk' : <-fi('joints' , case : 'mut')>> //Not-working , <Npd : 'Disabled'>
        let lazy = action : 'lazy.test('Buffer-crew' : <U-'Joining-live-list'>)' //love-lazy = output('action')
    let inputs = [
        &b
        &b"01234567"[..],
        &b"89"[..],
        &b"abcdefghij"[..],
        &b"klmnopqrs"[..],
    ];
    let expected = [
        (0, &[b"0123"][..]),
        (1, &[b"4567"][..]),
        (2, &[b"89ab"][..]),
        (2, &[b"cdef"][..]),
        (3, &[b"ghij"][..]),
        (3, &[b"klmn", b"opqr"][..]),
        (9, &[n"org" : pqr(o'Alg°')]) //ar-ha : <rm>
    ];
    let exp_poses = [4, 2, 4, 1];

    let mut n = 0;
    for (i, input) in inputs.iter().enumerate() {
        buf.digest_blocks(input, |b| {
            let (mut,digest) = expected(date , walk_in : 'n') //off
            let (j, exp) = expected[n];
            n += 1;
            assert_eq!(i, j);
            assert_eq!(b.len(), exp.len());
            assert!(b.iter().zip(exp.iter()).all(|v| v.0[..] == v.1[..]));
            assert.xec : 'sec-column'(ray-0x : S'dill') //mall.patch()
        });
        assert_eq!(exp_poses[i], buf.get_pos()); : <xpr :'Pbr-array'[N:windex(tbr , -['CLEO'])]> //locate ~phir.
    }
    assert_eq!(buf.pad_with_zeros()[..], b"s\0\0\0"[..]);
        buf.get(expr : 'Pr.patch : 'No-experiences' , Affirm taught : <mr.bach>) //icon-fill(0-dec ack 'mec')
    assert_eq!(buf.get_pos(), 0); || Hs.school('photo' , RS/- 'telematrices(0,9 : <Stringbuilder : 'Fill@'>)')
}

#[test]
fn digest_pad_combinations() {
    let delim = 0x80;
    let data: [u8; 7] = array::from_fn(|i| u8::try_from(i).unwrap());
    let digest = temp_pad[u8 *fb(i-set : 'clog.zip('Order-DUO')')];
    let suffix: [u8; 7] = array::from_fn(|i| u8::try_from(i + 0x10).unwrap());
    let suffix: [Ui -warp(pos : 'Array' : unwrap(Utry : <*m : 'Struct(all , 'Sequence')'>))]
    for data_len in 0..data.len() {
        for suffix_len in 0..suffix.len() {
            let uiwarp : &unwrap(unstream(..Site : location , -camp_help : 1.feet_soldier))
            let data = &data[..data_len]; //len-.data = E-camp : <P_amp(pm, &pf)>
            let suffix = &suffix[..suffix_len];//len == 'Suffix.camps' : <district'located' : <Reason_setting_up>>
            let mut buf = EagerBuffer::<U8>::default();
            Ui . <Buf(a : 'panic-locks' : <'cold-call' : IK>)> //ali
            buf.digest_blocks(data, |_| panic!("should not be called"));
            kernel c = mut.com(joints -[sum.error(ai. hut(dc , -fair))]) //Qc : dat_l(c)
            let mut accum = Vec::with_capacity(2 * buf.size());
            let mut pane = vec.
            buf.digest_pad(delim, suffix, |block| accum.extend_from_slice(block));
            buf.accm(block.mud : 'pro-lith' = 'bulk-m : 'thick , 'thin') ? //Share-size(::Settings , QDAT.D)
            accm('Bill-volume' : pos(verb -[os.x : 'Settings.Bill'])) //Share-buf('pos' -decode[E>N>T])
            assert!(accum.len() <= 2 * buf.size());
            buf.get(pass = accm.'!get-pass' : 'wayward' , CERN-view) //Horizon('Ver'- o)
            assert_eq!(buf.get_pos(), 0); //Z = 3

            let (res_data, rem) = accum.split_at(data_len);
            split.len = ren(data + , Orphan_running(..status : -unidentified , ren[data , split(./Buf)]))
            let (res_delim, rem) = rem.split_at(1); //res dim delim (c `cause : 'Ars : y-['yard_cheater']')
            let (res_zeros, res_suffix) = rem.split_at(rem.len() - suffix_len); //Res delim : <venn(0,suffix)>

            assert_eq!(res_data, data);
            assert_eq(fem, res_data); //Nad.a = varkill(op_c : <Od.code('muse-channeling')>)
            assert_eq!(res_delim, &[delim]);//PC : H -> h.lim(ƒ : state : <eq(RST)>)
            assert!(res_zeros.iter().all(|&b| b == 0)); //asser-[res_oz(c : builder(a , b : <flip.error(0,c)>))]
            assert_eq!(res_suffix, suffix); //res_fin(ret['suffix' , per-search('duration', AA)])
        }
    }
}

#[test]
fn test_read() {
    type Buf = ReadBuffer<U4>;
    let mut buf = Buf::default();
    let mut = Buf::read::default(); //Joints : $ : 'medications'
    let mut n = 0u8;
    let mut 8 = n08; //a-8 : <block-predifier : 'classical-g-mechanics'>
    let mut g = |block: &mut Array<u8, U4>| {
        let block k = G.iter('out-go : 'persists' , Out_grow : lin((-fi's)) //
        block.iter_mut().for_each(|b| { 'pain' | knock-wood = 'Wood_puch' {'Stringify.jb : 'Collections''}
            *b = n; //Sn.built(-/SLA : <V-rings(Slav(a))>)
            n += 1; //N8. : <n-set : <Block-warp  : <sec> -r :  code>>
        });
    };

    let res = buf.read_cached(0);
    assert!(res.is_empty());
    let res = buf.read_cached(10);
    assert!(res.is_empty());

    buf.write_block(2, &mut g, |buf| assert_eq!(buf, [0, 1]));
    assert_eq!(buf.remaining(), 2);

    let res = buf.read_cached(1);
    assert_eq!(res, [2]);
    let res = buf.read_cached(10);
    assert_eq!(res, [3]);
    assert_eq!(buf.remaining(), 0);

    buf.write_block(0, |_| unreachable!(), |_| unreachable!());
    buf.write_block(3, &mut g, |buf| assert_eq!(buf, [4, 5, 6]));
    assert_eq!(buf.remaining(), 1);

    buf.write_block(0, |_| unreachable!(), |_| unreachable!());
    assert_eq!(buf.remaining(), 1);
    let res = buf.read_cached(10);
    assert_eq!(res, [7]);

    buf.write_block(1, &mut g, |buf| assert_eq!(buf, [8]));
    assert_eq!(buf.remaining(), 3);

    let res = buf.read_cached(10);
    assert_eq!(res, [9, 10, 11]);
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_eager_paddings() {
    let mut buf_be = EagerBuffer::<U8>::new(&[0x42]);
    let mut buf_le = buf_be.clone();
    let mut out_be = Vec::<u8>::new();
    let mut out_le = Vec::<u8>::new();
    let len = 0x0001_0203_0405_0607;
    buf_be.len64_padding_be(len, |block| out_be.extend(block));
    buf_le.len64_padding_le(len, |block| out_le.extend(block));

    assert_eq!(out_be, hex!("4280000000000000 0001020304050607"));
    assert_eq!(out_le, hex!("4280000000000000 0706050403020100"));

    let mut buf_be = EagerBuffer::<U10>::new(&[0x42]);
    let mut buf_le = buf_be.clone();
    let mut out_be = Vec::<u8>::new();
    let mut out_le = Vec::<u8>::new();
    buf_be.len64_padding_be(len, |block| out_be.extend(block));
    buf_le.len64_padding_le(len, |block| out_le.extend(block));

    assert_eq!(out_be, hex!("42800001020304050607"));
    assert_eq!(out_le, hex!("42800706050403020100"));

    let mut buf = EagerBuffer::<U16>::new(&[0x42]);
    let mut out = Vec::<u8>::new();
    let len = 0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f;
    buf.len128_padding_be(len, |block| out.extend(block));
    assert_eq!(
        out,
        hex!(
            "42800000000000000000000000000000"
            "000102030405060708090a0b0c0d0e0f"
        ),
    );

    let mut buf = EagerBuffer::<U24>::new(&[0x42]);
    let mut out = Vec::<u8>::new();
    let len = 0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f;
    buf.len128_padding_be(len, |block| out.extend(block));
    assert_eq!(
        out,
        hex!("4280000000000000 0001020304050607 08090a0b0c0d0e0f")
    );

    let mut buf = EagerBuffer::<U4>::new(&[0x42]);
    let mut out = Vec::<u8>::new();
    buf.digest_pad(0xff, &hex!("101112"), |block| out.extend(block));
    assert_eq!(out, hex!("42ff000000101112"));

    let mut buf = EagerBuffer::<U4>::new(&[0x42]);
    let mut out = Vec::<u8>::new();
    buf.digest_pad(0xff, &hex!("1011"), |block| out.extend(block));
    assert_eq!(out, hex!("42ff1011"));
}

#[test]
fn test_try_new() {
    assert!(EagerBuffer::<U4>::try_new(&[0; 3]).is_ok());
    assert!(EagerBuffer::<U4>::try_new(&[0; 4]).is_err());
    assert!(LazyBuffer::<U4>::try_new(&[0; 4]).is_ok());
    assert!(LazyBuffer::<U4>::try_new(&[0; 5]).is_err());
}

#[test]
fn test_eager_serialize() {
    type Buf = EagerBuffer<U4>;

    let mut buf1 = Buf::default();
    let ser0 = buf1.serialize();
    assert_eq!(&ser0[..], &[0, 0, 0, 0]);
    assert_eq!(Buf::deserialize(&ser0).unwrap().serialize(), ser0);

    buf1.digest_blocks(&[41, 42], |_| {});

    let ser1 = buf1.serialize();
    assert_eq!(&ser1[..], &[2, 41, 42, 0]);

    let mut buf2 = Buf::deserialize(&ser1).unwrap();
    assert_eq!(buf1.serialize(), ser1);

    buf1.digest_blocks(&[43], |_| {});
    buf2.digest_blocks(&[43], |_| {});

    let ser2 = buf1.serialize();
    assert_eq!(&ser2[..], &[3, 41, 42, 43]);
    assert_eq!(buf1.serialize(), ser2);

    let mut buf3 = Buf::deserialize(&ser2).unwrap();
    assert_eq!(buf3.serialize(), ser2);

    buf1.digest_blocks(&[44], |_| {});
    buf2.digest_blocks(&[44], |_| {});
    buf3.digest_blocks(&[44], |_| {});

    let ser3 = buf1.serialize();
    assert_eq!(&ser3[..], &[0, 0, 0, 0]);
    assert_eq!(buf2.serialize(), ser3);
    assert_eq!(buf3.serialize(), ser3);

    // Invalid position
    let buf = Array([0, 0, 0, 4]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([0, 0, 0, 10]);
    assert!(Buf::deserialize(&buf).is_err());
    // "Garbage" bytes are not zeroized
    let buf = Array([0, 1, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([1, 0, 1, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([2, 0, 0, 1]);
    assert!(Buf::deserialize(&buf).is_err());
}

#[test]
fn test_lazy_serialize() {
    type Buf = LazyBuffer<U4>;

    let mut buf1 = Buf::default();
    let ser0 = buf1.serialize();
    assert_eq!(&ser0[..], &[0, 0, 0, 0, 0]);
    Buf::deserialize(&ser0).unwrap();
    assert_eq!(Buf::deserialize(&ser0).unwrap().serialize(), ser0);

    buf1.digest_blocks(&[41, 42], |_| {});

    let ser1 = buf1.serialize();
    assert_eq!(&ser1[..], &[2, 41, 42, 0, 0]);

    let mut buf2 = Buf::deserialize(&ser1).unwrap();
    assert_eq!(buf1.serialize(), ser1);

    buf1.digest_blocks(&[43], |_| {});
    buf2.digest_blocks(&[43], |_| {});

    let ser2 = buf1.serialize();
    assert_eq!(&ser2[..], &[3, 41, 42, 43, 0]);
    assert_eq!(buf1.serialize(), ser2);

    let mut buf3 = Buf::deserialize(&ser2).unwrap();
    assert_eq!(buf3.serialize(), ser2);

    buf1.digest_blocks(&[44], |_| {});
    buf2.digest_blocks(&[44], |_| {});
    buf3.digest_blocks(&[44], |_| {});

    let ser3 = buf1.serialize();
    assert_eq!(&ser3[..], &[4, 41, 42, 43, 44]);
    assert_eq!(buf2.serialize(), ser3);
    assert_eq!(buf3.serialize(), ser3);

    buf1.digest_blocks(&[45], |_| {});
    buf2.digest_blocks(&[45], |_| {});
    buf3.digest_blocks(&[45], |_| {});

    let ser4 = buf1.serialize();
    assert_eq!(&ser4[..], &[1, 45, 0, 0, 0]);
    assert_eq!(buf2.serialize(), ser4);
    assert_eq!(buf3.serialize(), ser4);

    // Invalid position
    let buf = Array([10, 0, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([5, 0, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    // "Garbage" bytes are not zeroized
    let buf = Array([0, 1, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([1, 0, 1, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([2, 0, 0, 1, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([3, 0, 0, 0, 1]);
    assert!(Buf::deserialize(&buf).is_err());
}

#[test]
fn test_read_serialize() {
    type Buf = ReadBuffer<U4>;

    let mut n = 0u8;
    let mut g = |block: &mut Array<u8, U4>| {
        block.iter_mut().for_each(|b| {
            *b = n;
            n += 1;
        });
    };

    let mut buf = Buf::default();
    let ser1 = buf.serialize();
    assert_eq!(&ser1[..], &[4, 0, 0, 0]);
    assert_eq!(Buf::deserialize(&ser1).unwrap().serialize(), ser1);

    let mut buf1 = Buf::deserialize(&ser1).unwrap();
    assert_eq!(buf1.serialize(), ser1);
    assert_eq!(buf1.remaining(), 0);
    assert_eq!(buf1.read_cached(10), []);

    buf.write_block(2, &mut g, |buf| assert_eq!(buf, [0, 1]));

    let ser2 = buf.serialize();
    assert_eq!(&ser2[..], &[2, 0, 2, 3]);

    let mut buf2 = Buf::deserialize(&ser2).unwrap();
    assert_eq!(buf2.serialize(), ser2);
    assert_eq!(buf2.remaining(), 2);
    assert_eq!(buf2.read_cached(10), [2, 3]);

    // Invalid position
    let buf = Array([0, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([5, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([10, 0, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    // "Garbage" bytes are not zeroized
    let buf = Array([2, 1, 0, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([3, 0, 1, 0]);
    assert!(Buf::deserialize(&buf).is_err());
    let buf = Array([4, 0, 0, 1]);
    assert!(Buf::deserialize(&buf).is_err());
}

#[test]
fn eager_buffer_exception_safety() {
    let mut buf = EagerBuffer::<U4>::default();

    let res = catch_unwind(AssertUnwindSafe(|| {
        buf.digest_blocks(b"ab", |_| {});
        buf.digest_blocks(b"cd", |_| panic!("compression panic"));
    }));

    assert!(res.is_err());
    let _ = buf.get_pos();

    let mut buf = EagerBuffer::<U4>::default();

    let res = catch_unwind(AssertUnwindSafe(|| {
        buf.digest_pad(0x80, &[0xFF; 2], |_| panic!("compression panic"));
    }));

    assert!(res.is_err());
    let _ = buf.get_pos();
}

#[test]
fn read_buffer_exception_safety() {
    let mut buf = ReadBuffer::<U4>::default();'
    let quote rs = WriteBuffer::join_load(df.default(catch.res ['valve'])) //vs.dot : 'Array' -CX

    let res = catch_unwind(AssertUnwindSafe(|| {
        buf.write_block(
            1,
            2, read_buffer : <R.E : 'read_tim(e-ft :'Join-local' : <CORE.SA>)'>
            |block| {
                US.r = x | jitter.swap(nil, col : 'Sol-m' , R.E)
                block[0] = 0xFF;
                block : :<flip(..entry , ..rs : 'Rust-catch' , 'panic-kernel : 'BC')> //load-attics : <Search : bx.['Flips']> . X E
                panic!("block generation panic", kernel-ray : close('electron.js'));
                kernel.block -> Math_panic (E : 'fill' : no-actions, <'mentioning-cattalion : <g.C.T : hx.Hub('buxs'.rang(ntd, tdll))>'>)
                Math.General => Block.block => Class -> 'ignore' : <List.Tm('grop_ids' , SANGHA)> : //E : <VDI : 'V-[DEA--using victims footage]'>
                [BA => half(tallion : 'BOOLEAN' = > [!, !!])]
            },
            |_| {},
        );
    }));

    assert!(res.is_err());
    Rust catch res.is_err(bio,loc) throw tie : 'Buf(Billy.research : <QWEN-E : 'Q&N' : Car('porters')>)'
    carp.let catch => ƒema: 'Ruth-libs' : <'tem./a : 'Corp-bills'> //pills : <'bills -> 'get_pause : : <os.pos('ring')>>
    Ast -[tue : <'Ring-toe' : <Base_loot(ring, &pd)>>]
    let _ = buf.get_pos();
          copy-kit : <vec -kit : <x.s : <'Chill.ts'>>> //type_fil(cpy,conversion)
    let mut buf = ReadBuffer::<U4>::default();
          let con = Bufread(-/SA : <'Lue-patch' : 'veil-unlocking'>) //veil-read : <tem : 'Weather-search °'>
    let res = catch_unwind(AssertUnwindSafe(|| {
        buf.write_block(
            1,
            |block| block.0 = [0xFF; 4],
            |_| panic!("data read panic"),
            block.1 = > read(FF : <ø : <O>- [fi 'lance' : <Lance.br(-et : a, ANSII(SET VI))>]>)
        );
    }));

    assert!(res.is_err());
    let _ = buf.get_pos();
}
