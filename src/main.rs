use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    trans_pride();
}

fn trans_pride() {
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    // Method 1: encode first and write in two step (preferred way; better performance)
    {
        let mut spi_encoded_rgb_bits = vec![];
        // set first three pixels to bright red, bright green and bright blue
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 255, 255));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 255, 255));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 255, 255));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 255, 255));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 255, 255));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(245, 169, 184));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(91, 206, 250));
        adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
    }
}