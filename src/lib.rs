pub mod api {
    use std::collections::HashMap;

    /*** Hardware-specifics ***/
    /// CPU sockets.
    pub enum CPUSocket {
        // Intel Desktop Sockets
        LGA1151,   // 8th & 9th Gen
        LGA1155,   // 2nd & 3rd Gen
        LGA1200,   // 10th & 11th Gen
        LGA1700,   // 12th & 13th Gen
        LGA1366,   // Older HEDT
        LGA2011,   // Enthusiast (X79, X99 platforms)
        LGA2066,   // Enthusiast/Core X-series
    
        // Intel Server/Workstation
        LGA4189,   // Intel Xeon
        LGA4677,   // Xeon Sapphire Rapids
    
        // AMD Desktop Sockets
        AM3,       // Phenom II, FX
        AM3Plus,   // Updated AM3
        AM4,       // Ryzen 1000–5000
        AM5,       // Ryzen 7000 and later
    
        // AMD HEDT/Workstation
        TR4,       // Threadripper 1000–2000
        STRX4,     // Threadripper 3000
        SWRX8,     // Threadripper Pro 3000/5000
    
        // AMD Server Sockets
        SP3,       // EPYC
    
        // Miscellaneous Sockets
        BGA,       // Soldered CPUs (generic)
        ARM,       // ARM-based CPUs (e.g., Raspberry Pi, mobile)
        Other,     // Placeholder for less common or future sockets
    }

    /// Types of RAM.
    pub enum RAMType {
        SDR,       // Single Data Rate (legacy)
        DDR,       // DDR1 (legacy)
        DDR2,      // DDR2 (legacy)
        DDR3,      // DDR3
        DDR4,      // DDR4
        DDR5,      // DDR5
        LPDDR3,    // Low-power DDR3 (mobile devices)
        LPDDR4,    // Low-power DDR4 (mobile devices)
        LPDDR5,    // Low-power DDR5 (mobile devices)
        Other,     // Placeholder for uncommon or future RAM types
    }

    /*** Parts ***/
    /// Basic part information.
    pub struct PartMetadata {
        pub model: String,
        pub manufacturer: String,
        pub price: u16,
        id: String
    }

    impl PartMetadata {
        /// Get this part's ID.
        pub fn get_id(&self) -> &String {
            &self.id
        }
    }

    /// A motherboard.
    pub struct Motherboard {
        pub base: PartMetadata,
        pub socket: CPUSocket,
        pub chipset: String,
        pub ram_type: RAMType,
        pub ram_slots: u8,
        used_ram_slots: u8,
        pub pci_slots: u8,
        used_pci_slots: u8
    }

    impl Motherboard {
        /// Create a new motherboard.
        pub fn new(
            base: PartMetadata,
            socket: CPUSocket,
            chipset: String,
            ram_type: RAMType,
            ram_slots: u8,
            pci_slots: u8
        ) -> Motherboard {
            Motherboard {
                base,
                socket,
                chipset: chipset.to_string(),
                ram_type,
                ram_slots,
                used_ram_slots: 0,
                pci_slots,
                used_pci_slots: 0
            }
        }

        /// Get the amount of used RAM slots.
        pub fn get_used_ram_slots(&self) -> u8 {
            self.used_ram_slots.clone()
        }

        /// Set the amount of used RAM slots.
        pub fn set_used_ram_slots(&mut self, new_amount: u8) -> Result<(), &'static str>{
            if self.ram_slots < new_amount {
                self.used_ram_slots = new_amount;
                Ok(())
            } else {
                Err("The new amount will exceed the maximum slots defined by the motherboard's specifications.")
            }
        }

        /// Get the amount of used PCI slots.
        pub fn get_used_pci_slots(&self) -> u8 {
            self.used_pci_slots.clone()
        }

        /// Set the amount of used PCI slots.
        pub fn set_used_pci_slots(&mut self, new_amount: u8) -> Result<(), &'static str>{
            if self.pci_slots < new_amount {
                self.used_pci_slots = new_amount;
                Ok(())
            } else {
                Err("The new amount will exceed the maximum slots defined by the motherboard's specifications.")
            }
        }
    }

    /// Some PC parts.
    pub enum Part {
        Motherboard(Motherboard)
    }

    /*** Other ***/
    /// UUID for identifying parts.
    pub struct Uuid {
        /// The actual UUID.
        pub id: String
    }

    /// A PC.
    pub struct Machine {
        /// The name of this machine.
        pub name: String,

        /// The components of this machine.
        parts: HashMap<Uuid, Part>
    }
    
    impl Machine {
        /// Get the current part hashmap of this machine.
        pub fn get_parts(&self) -> &HashMap<Uuid, Part> {
            &self.parts
        }

        // TODO: Add UUID collision checking
        /// Add a new part to this machine.
        pub fn add_part(&mut self, part: Part) {
            self.parts.
        }

        /// Remove a part from this machine.
        pub fn remove_part(&mut self) {}
    }
}