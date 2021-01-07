use clap::Clap;

#[derive(Clap)]
#[clap(
    name = "primes_sieve",
    about = "prints the number of prime numbers not greater than the given number"
)]
struct Options {
    #[clap(short, long, about = "print discovered prime numbers")]
    debug: bool,
    #[clap(about = "find all the prime numbers not greater than this one")]
    number: usize,
}

fn main() {
    let opts = Options::parse();
    let primes = find_prime_numnbers(opts.number);
    println!("Primes in the range: {}", primes.len());
    if opts.debug {
        println!("{:#?}", primes);
    }
}

fn find_prime_numnbers(number: usize) -> Vec<usize> {
    let sieve = create_sieve_of_eratosthenes(number);
    return select_prime_numbers(sieve);
}

fn create_sieve_of_eratosthenes(number: usize) -> Vec<bool> {
    let mut sieve = vec![true; number + 1];
    let number_sqrt = (number as f64).sqrt() as usize;
    for i in 2..=number_sqrt {
        if sieve[i] {
            for j in (i * i..=number).step_by(i) {
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
