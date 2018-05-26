use std::io;
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(PartialEq)]
enum BankingType {
    NoBanking,
    MBC1,
    MBC2
}

pub struct Cartridge {
    rom: Vec<u8>,
    ram_banks: [u8; 0x8000],
    banking_type: BankingType,
    current_rom_bank: u8,
    current_ram_bank: u8,
    rom_banking_mode: bool,
    pub ram_write_enabled: bool
}

impl Cartridge {

    /// Default constructor.
    pub fn new() -> Cartridge {
        let mut cartridge = Cartridge {
            rom: Vec::new(),
            ram_banks: [0; 0x8000],
            banking_type: BankingType::NoBanking,
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom_banking_mode: true,
            ram_write_enabled: false
        };

        // Set rom to bytes from file
        let args: Vec<String> = env::args().collect();
        let rom = cartridge.read_rom(&args[1]);
        let rom = match rom {
            Ok(rom) => rom,
            Err(e) => panic!("{}", e),
        };
        cartridge.rom = rom;

        // Set rom banking type
        match cartridge.rom[0x147] {
            0 => cartridge.banking_type = BankingType::NoBanking,
            1 | 2 | 3 => cartridge.banking_type = BankingType::MBC1,
            4 | 5 | 6 => cartridge.banking_type = BankingType::MBC2,
            _ => panic!("Banking type is currently not supported. Value at 0x147 was 0x{:02X}", cartridge.rom[0x147])
        }
        cartridge
    }

    /// Reads a rom file's bytes to a vector on success.
    pub fn read_rom(&mut self, location: &str) -> io::Result<Vec<u8>> {
        let mut rom = File::open(location)?;
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer)?;
        
        // Panic if ROM has more bytes than possible
        // or is amount of bytes is not a power of two
        if buffer.len() > 0x200000 || (buffer.len() & (buffer.len() - 1)) != 0 {
            panic!("Invalid ROM size, {} bytes", buffer.len());
        }
        Ok(buffer)
    }

    /// Updates ability to write to RAM based on
    /// the value of the lower half of address.
    pub fn update_ram_writing(&mut self, address: u16, byte: u8) {

        // Stop if using MBC2 and the 4th address bit is 1
        if self.banking_type == BankingType::MBC2 {
            if (address & 0x08) >> 3 == 1 {
                return;
            }
        }

        if (byte & 0x0F) == 0x0A {
            self.ram_write_enabled = true;
        }
        else if (byte & 0x0F) == 0x00 {
            self.ram_write_enabled = false;
        }
    }    

    /// Changes lower bits of the current ROM bank.
    pub fn change_lo_rom_bank(&mut self, byte: u8) {
        
        // Set bank to lower half of byte if MBC2
        if self.banking_type == BankingType::MBC2 {
            self.current_rom_bank = byte & 0x0F;
            if self.current_rom_bank == 0 {
                self.current_rom_bank += 1;
            }
            return;
        }

        // Sets bank's first five bits otherwise
        self.current_rom_bank &= 0xE0;
        self.current_rom_bank |= byte & 0x1F;
        if self.current_rom_bank == 0 {
            self.current_rom_bank += 1;
        }
    }

    /// Sets ROM bank's upper 3 bits to the upper 3
    /// bits of byte.
    pub fn change_hi_rom_bank(&mut self, byte: u8) {
        self.current_rom_bank &= 0x1F;
        let upper_three = byte & 0xE0;
        self.current_rom_bank |= upper_three;
        if self.current_rom_bank == 0 {
            self.current_rom_bank += 1;
        }
    }

    /// Sets RAM bank to lower 2 bits of byte.
    pub fn change_ram_bank(&mut self, byte: u8) {
        self.current_ram_bank = byte & 0x03;
    }

    /// Determines if ROM or RAM banking mode should
    /// be used based on the LSB of byte.
    pub fn set_banking_mode(&mut self, byte: u8) {
        self.rom_banking_mode = if (byte & 0x01) == 0 {
            true
        } 
        else {
            false
        };

        // Update RAM bank to 0 if in ROM banking mode
        if self.rom_banking_mode {
            self.current_ram_bank = 0;
        }
    }

    /// Handles banks based upon the address given.
    pub fn manage_banking(&mut self, address: u16, byte: u8) {

        // Enable RAM bank writes
        if address < 0x2000 {
            if self.banking_type == BankingType::MBC1 || self.banking_type == BankingType::MBC2 {
                self.update_ram_writing(address, byte);
            }
        }

        // Change low bits of ROM bank
        else if address >= 0x2000 && address < 0x4000 {
            if self.banking_type == BankingType::MBC1 || self.banking_type == BankingType::MBC2 {
                self.change_lo_rom_bank(byte);
            } 
        }

        // Change RAM bank or change high bits of ROM bank
        else if address >= 0x4000 && address < 0x6000 {
            if self.banking_type == BankingType::MBC1 {
                if self.rom_banking_mode {
                    self.change_hi_rom_bank(byte);
                }
                else {
                    self.change_ram_bank(byte);
                }
            }
        }

        // Update banking mode
        else if address >= 0x6000 && address < 0x8000 {
            if self.banking_type == BankingType::MBC1 {
                self.set_banking_mode(byte);
            }
        }
    }

    /// Returns the byte in rom at a given address.
    pub fn get_rom(&mut self, address: u32) -> u8 {
        self.rom[address as usize]
    }

    /// Returns the byte in a ram bank at a given address.
    pub fn get_ram(&mut self, address: u16) -> u8 {
        self.ram_banks[address as usize]
    }

    /// Sets the byte in a ram bank at a given address.
    pub fn set_ram(&mut self, address: u16, byte: u8) {
        self.ram_banks[address as usize] = byte;
    }

    /// Getter for the current rom bank.
    pub fn get_current_rom_bank(&mut self) -> u8 {
        self.current_rom_bank
    }

    /// Getter for the current ram bank.
    pub fn get_current_ram_bank(&mut self) -> u8 {
        self.current_ram_bank
    }
}