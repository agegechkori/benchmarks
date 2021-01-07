use clap::Clap;

#[derive(Clap)]
#[clap(
    name = "primes_sieve",
    about = "counts the prime numbers not greater than the given number"
)]
struct Options {
    #[clap(short, long, about = "print discovered prime numbers")]
    debug: bool,
    #[clap(about = "upper bound for the prime numbers search")]
    upper_bound: usize,
}

fn main() {
    let opts = Options::parse();
    let primes = find_prime_numnbers(opts.upper_bound);
    println!(
        "Found {} prime numbers not greater than {}",
        primes.len(),
        opts.upper_bound
    );
    if opts.debug {
        println!("{:#?}", primes);
    }
}

fn find_prime_numnbers(upper_bound: usize) -> Vec<usize> {
    let sieve = create_sieve_of_eratosthenes(upper_bound);
    return select_prime_numbers(sieve);
}

fn create_sieve_of_eratosthenes(upper_bound: usize) -> Vec<bool> {
    let mut sieve = vec![true; upper_bound + 1];
    let upper_bound_integer_sqrt = (upper_bound as f64).sqrt() as usize;
    for i in 2..=upper_bound_integer_sqrt {
        if sieve[i] {
            for j in (i * i..=upper_bound).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    return sieve;
}

fn select_prime_numbers(sieve: Vec<bool>) -> Vec<usize> {
    let mut primes: Vec<usize> = vec![];
    for i in 2..sieve.len() {
        if sieve[i] {
            primes.push(i);
        }
    }
    return primes;
}
