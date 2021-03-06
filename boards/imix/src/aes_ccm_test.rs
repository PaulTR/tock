use capsules::aes_ccm;
use capsules::test::aes_ccm::Test;
use kernel::hil::symmetric_encryption::{AES128, AES128CCM, AES128_BLOCK_SIZE};
use sam4l::aes::{Aes, AES};

pub unsafe fn run() {
    let ccm = static_init_ccm();
    AES.set_client(ccm);

    let t = static_init_test(ccm);
    ccm.set_client(t);

    t.run();
}

unsafe fn static_init_ccm() -> &'static mut aes_ccm::AES128CCM<'static, Aes<'static>> {
    const CRYPT_SIZE: usize = 7 * AES128_BLOCK_SIZE;
    let crypt_buf = static_init!([u8; CRYPT_SIZE], [0x00; CRYPT_SIZE]);
    static_init!(
        aes_ccm::AES128CCM<'static, Aes<'static>>,
        aes_ccm::AES128CCM::new(&AES, crypt_buf)
    )
}

type AESCCM = aes_ccm::AES128CCM<'static, Aes<'static>>;

unsafe fn static_init_test(aes_ccm: &'static AESCCM) -> &'static mut Test<'static, AESCCM> {
    let data = static_init!([u8; 4 * AES128_BLOCK_SIZE], [0x00; 4 * AES128_BLOCK_SIZE]);
    static_init!(Test<'static, AESCCM>, Test::new(aes_ccm, data))
}
