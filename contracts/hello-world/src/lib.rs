#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct VaccineCertificate {
    pub cert_id: u64,
    pub person_name: String,
    pub vaccine_name: String,
    pub vaccination_date: u64,
    pub verified: bool,
}

#[contracttype]
pub enum CertificateRegistry {
    Cert(u64),
}

const TOTAL_CERTS: Symbol = symbol_short!("T_CERT");

#[contract]
pub struct VaccineCertificateVerifier;

#[contractimpl]
impl VaccineCertificateVerifier {
    pub fn issue_certificate(env: Env, person_name: String, vaccine_name: String) -> u64 {
        let mut total: u64 = env.storage().instance().get(&TOTAL_CERTS).unwrap_or(0);
        total += 1;

        let cert = VaccineCertificate {
            cert_id: total,
            person_name,
            vaccine_name,
            vaccination_date: env.ledger().timestamp(),
            verified: false,
        };

        env.storage().instance().set(&CertificateRegistry::Cert(total), &cert);
        env.storage().instance().set(&TOTAL_CERTS, &total);
        env.storage().instance().extend_ttl(5000, 5000);

        total
    }

    pub fn verify_certificate(env: Env, cert_id: u64) {
        let mut cert: VaccineCertificate = env
            .storage()
            .instance()
            .get(&CertificateRegistry::Cert(cert_id))
            .unwrap_or_else(|| panic!("Certificate not found"));

        cert.verified = true;

        env.storage().instance().set(&CertificateRegistry::Cert(cert_id), &cert);
        env.storage().instance().extend_ttl(5000, 5000);
    }

    pub fn view_certificate(env: Env, cert_id: u64) -> VaccineCertificate {
        env.storage()
            .instance()
            .get(&CertificateRegistry::Cert(cert_id))
            .unwrap_or(VaccineCertificate {
                cert_id: 0,
                person_name: String::from_str(&env, "Not_Found"),
                vaccine_name: String::from_str(&env, "Not_Found"),
                vaccination_date: 0,
                verified: false,
            })
    }

    pub fn total_certificates(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_CERTS).unwrap_or(0)
    }
}
