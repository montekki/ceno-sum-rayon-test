use either::Either;
use ff_ext::GoldilocksExt2;
use itertools::Itertools;
use multilinear_extensions::{
    mle::MultilinearExtension, monomial::Term, virtual_polys::VirtualPolynomials,
};
use p3::field::FieldAlgebra;
use rand::thread_rng;
use sumcheck::structs::IOPProverState;
use tracing::Level;
use transcript::BasicTranscript as Transcript;

const NUM_DEGREE: usize = 3;
const NV: usize = 23;

type E = GoldilocksExt2;

fn main() {
    let _guard = tracing_profile::init_tracing().unwrap();

    let nv = NV;

    let mut rng = thread_rng();
    let fs = (0..NUM_DEGREE)
        .map(|_| MultilinearExtension::<E>::random(nv, &mut rng))
        .collect_vec();

    rayon::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|_scope_1| {
                let num_threads = usize::next_power_of_two(rayon::current_num_threads()) / 2;
                println!("current num threads {num_threads}");
                let virtual_poly_v2 = VirtualPolynomials::new_from_monimials(
                    num_threads,
                    nv,
                    vec![Term {
                        scalar: Either::Right(E::ONE),
                        product: fs.iter().map(Either::Left).collect_vec(),
                    }],
                );

                let mut prover_transcript = Transcript::new(b"test");
                let my_span = tracing::span!(Level::INFO, "proving").entered();

                let (_proof, state) =
                    IOPProverState::<E>::prove(virtual_poly_v2, &mut prover_transcript);

                my_span.exit();
                println!("s1 challenges {:?}", state.collect_raw_challenges());
            });
        }
    });
}
