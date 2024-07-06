pub mod note;

use defmt::{debug, warn};
use stm32f4xx_hal::otg_fs::{UsbBus, UsbBusType, USB};
use usb_device::{
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_midi::{
    data::{
        usb::constants::{USB_AUDIO_CLASS, USB_MIDISTREAMING_SUBCLASS},
        usb_midi::{
            midi_packet_reader::MidiPacketBufferReader, usb_midi_event_packet::UsbMidiEventPacket,
        },
    },
    midi_device::MidiClass,
};

pub struct UsbMidi<'a> {
    midi: MidiClass<'a, UsbBusType>,
    usb_dev: UsbDevice<'a, UsbBusType>,
}

impl<'a> UsbMidi<'a> {
    pub fn new(usb_bus: &'a UsbBusAllocator<UsbBusType>) -> Self {
        let midi = MidiClass::new(usb_bus, 1, 1).unwrap();
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x666, 0x666))
            .device_class(USB_AUDIO_CLASS)
            .device_sub_class(USB_MIDISTREAMING_SUBCLASS)
            // .self_powered(true)
            .strings(&[StringDescriptors::default()
                .manufacturer("paw-music")
                .product("PAW1")
                .serial_number("TEST")])
            .unwrap()
            .build();

        Self { midi, usb_dev }
    }

    pub fn poll(&mut self, f: impl Fn(UsbMidiEventPacket)) {
        if self.usb_dev.poll(&mut [&mut self.midi]) {
            let mut buffer = [0; 64];

            if let Ok(size) = self.midi.read(&mut buffer) {
                let buffer_reader = MidiPacketBufferReader::new(&buffer, size);

                for packet in buffer_reader.into_iter() {
                    match packet {
                        Ok(packet) => {
                            debug!("MIDI Packet: {}", format!("{:?}", packet).as_str());

                            f(packet);
                        }
                        Err(err) => {
                            warn!("MIDI Packet ERROR: {}", format!("{:?}", err).as_str());
                        }
                    }
                }
            }
        }
    }
}
