use either::Either;
use ff_ext::GoldilocksExt2;
use itertools::Itertools;
use multilinear_extensions::{
    mle::MultilinearExtension, monomial::Term, virtual_polys::VirtualPolynomials,
};
use p3::field::FieldAlgebra;
use rand::thread_rng;
use sumcheck::structs::IOPProverState;
use transcript::BasicTranscript as Transcript;

const NUM_DEGREE: usize = 3;
const NV: usize = 23;

type E = GoldilocksExt2;

fn main() {
    let nv = NV;

    let mut rng = thread_rng();
    let fs = (0..NUM_DEGREE)
        .map(|_| MultilinearExtension::<E>::random(nv, &mut rng))
        .collect_vec();

    rayon::in_place_scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|_scope_1| {
                let virtual_poly_v2 = VirtualPolynomials::new_from_monimials(
                    rayon::current_num_threads(),
                    nv,
                    vec![Term {
                        scalar: Either::Right(E::ONE),
                        product: fs.iter().map(Either::Left).collect_vec(),
                    }],
                );

                let mut prover_transcript = Transcript::new(b"test");
                let (_proof, state) =
                    IOPProverState::<E>::prove(virtual_poly_v2, &mut prover_transcript);
                println!("s1 challenges {:?}", state.collect_raw_challenges());
            });
        }
    });
}
