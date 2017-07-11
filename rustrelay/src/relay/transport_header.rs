use super::ipv4_header::{IPv4HeaderData, Protocol};
use super::tcp_header::{TCPHeader, TCPHeaderData, TCPHeaderMut};
use super::udp_header::{UDPHeader, UDPHeaderData, UDPHeaderMut, UDP_HEADER_LENGTH};

pub enum TransportHeader<'a> {
    TCP(TCPHeader<'a>),
    UDP(UDPHeader<'a>),
}

pub enum TransportHeaderMut<'a> {
    TCP(TCPHeaderMut<'a>),
    UDP(UDPHeaderMut<'a>),
}

#[derive(Clone)]
pub enum TransportHeaderData {
    TCP(TCPHeaderData),
    UDP(UDPHeaderData),
}

#[allow(dead_code)]
impl TransportHeaderData {
    pub fn parse(protocol: Protocol, raw: &[u8]) -> Option<Self> {
        match protocol {
            Protocol::UDP => Some(UDPHeaderData::parse(raw).into()),
            Protocol::TCP => Some(TCPHeaderData::parse(raw).into()),
            _ => None
        }
    }

    #[inline]
    pub fn bind<'c, 'a: 'c, 'b: 'c>(&'a self, raw: &'b [u8]) -> TransportHeader<'c> {
        TransportHeader::new(raw, self)
    }

    #[inline]
    pub fn bind_mut<'c, 'a: 'c, 'b: 'c>(&'a mut self, raw: &'b mut [u8]) -> TransportHeaderMut<'c> {
        TransportHeaderMut::new(raw, self)
    }

    #[inline]
    pub fn source_port(&self) -> u16 {
        match *self {
            TransportHeaderData::TCP(ref tcp_header_data) => tcp_header_data.source_port(),
            TransportHeaderData::UDP(ref udp_header_data) => udp_header_data.source_port(),
        }
    }

    #[inline]
    pub fn destination_port(&self) -> u16 {
        match *self {
            TransportHeaderData::TCP(ref tcp_header_data) => tcp_header_data.destination_port(),
            TransportHeaderData::UDP(ref udp_header_data) => udp_header_data.destination_port(),
        }
    }

    #[inline]
    pub fn header_length(&self) -> u8 {
        match *self {
            TransportHeaderData::TCP(ref tcp_header_data) => tcp_header_data.header_length(),
            TransportHeaderData::UDP(_) => UDP_HEADER_LENGTH,
        }
    }
}

impl<'a> TransportHeader<'a> {
    pub fn new(raw: &'a [u8], data: &'a TransportHeaderData) -> Self {
        match *data {
            TransportHeaderData::TCP(ref tcp_header_data) => tcp_header_data.bind(raw).into(),
            TransportHeaderData::UDP(ref udp_header_data) => udp_header_data.bind(raw).into(),
        }
    }
}

impl<'a> TransportHeaderMut<'a> {
    pub fn new(raw: &'a mut [u8], data: &'a mut TransportHeaderData) -> Self {
        match *data {
            TransportHeaderData::TCP(ref mut tcp_header_data) => tcp_header_data.bind_mut(raw).into(),
            TransportHeaderData::UDP(ref mut udp_header_data) => udp_header_data.bind_mut(raw).into(),
        }
    }
}

// shared definition for TransportHeader and TransportHeaderMut
macro_rules! transport_header_common {
    ($name:ident, $raw_type:ty, $data_type:ty) => {
        // for readability, declare structs manually outside the macro
        #[allow(dead_code)]
        impl<'a> $name<'a> {
            #[inline]
            pub fn raw(&self) -> &[u8] {
                match *self {
                    $name::TCP(ref tcp_header) => tcp_header.raw(),
                    $name::UDP(ref udp_header) => udp_header.raw(),
                }
            }

            #[inline]
            pub fn data_clone(&self) -> TransportHeaderData {
                match *self {
                    $name::TCP(ref tcp_header) => tcp_header.data().clone().into(),
                    $name::UDP(ref udp_header) => udp_header.data().clone().into(),
                }
            }

            #[inline]
            pub fn source_port(&self) -> u16 {
                match *self {
                    $name::TCP(ref tcp_header) => tcp_header.data().source_port(),
                    $name::UDP(ref udp_header) => udp_header.data().source_port(),
                }
            }

            #[inline]
            pub fn destination_port(&self) -> u16 {
                match *self {
                    $name::TCP(ref tcp_header) => tcp_header.data().destination_port(),
                    $name::UDP(ref udp_header) => udp_header.data().destination_port(),
                }
            }

            #[inline]
            pub fn header_length(&self) -> u8 {
                match *self {
                    $name::TCP(ref tcp_header) => tcp_header.data().header_length(),
                    $name::UDP(_) => UDP_HEADER_LENGTH,
                }
            }
        }
    }
}

transport_header_common!(TransportHeader, &'a [u8], &'a TransportHeaderData);
transport_header_common!(TransportHeaderMut, &'a mut [u8], &'a mut TransportHeaderData);

// additional methods for the mutable version
#[allow(dead_code)]
impl<'a> TransportHeaderMut<'a> {
    #[inline]
    pub fn raw_mut(&mut self) -> &mut [u8] {
        match *self {
            TransportHeaderMut::TCP(ref mut tcp_header) => tcp_header.raw_mut(),
            TransportHeaderMut::UDP(ref mut udp_header) => udp_header.raw_mut(),
        }
    }

    #[inline]
    pub fn swap_source_and_destination(&mut self) {
        match *self {
            TransportHeaderMut::TCP(ref mut tcp_header) => tcp_header.swap_source_and_destination(),
            TransportHeaderMut::UDP(ref mut udp_header) => udp_header.swap_source_and_destination(),
        }
    }

    #[inline]
    pub fn set_payload_length(&mut self, payload_length: u16) {
        match *self {
            TransportHeaderMut::UDP(ref mut udp_header) => udp_header.set_payload_length(payload_length),
            _ => (), // TCP does not store its payload length
        }
    }

    #[inline]
    pub fn update_checksum(&mut self, ipv4_header_data: &IPv4HeaderData, payload: &[u8]) {
        match *self {
            TransportHeaderMut::TCP(ref mut tcp_header) => tcp_header.update_checksum(ipv4_header_data, payload),
            TransportHeaderMut::UDP(ref mut udp_header) => udp_header.update_checksum(ipv4_header_data, payload),
        }
    }
}

impl From<TCPHeaderData> for TransportHeaderData {
    fn from(tcp_header_data: TCPHeaderData) -> TransportHeaderData {
        TransportHeaderData::TCP(tcp_header_data)
    }
}

impl From<UDPHeaderData> for TransportHeaderData {
    fn from(udp_header_data: UDPHeaderData) -> TransportHeaderData {
        TransportHeaderData::UDP(udp_header_data)
    }
}

impl<'a> From<TCPHeader<'a>> for TransportHeader<'a> {
    fn from(tcp_header: TCPHeader) -> TransportHeader {
        TransportHeader::TCP(tcp_header)
    }
}

impl<'a> From<UDPHeader<'a>> for TransportHeader<'a> {
    fn from(udp_header: UDPHeader) -> TransportHeader { 
        TransportHeader::UDP(udp_header)
    }
}

impl<'a> From<TCPHeaderMut<'a>> for TransportHeaderMut<'a> {
    fn from(tcp_header: TCPHeaderMut) -> TransportHeaderMut {
        TransportHeaderMut::TCP(tcp_header)
    }
}

impl<'a> From<UDPHeaderMut<'a>> for TransportHeaderMut<'a> {
    fn from(udp_header: UDPHeaderMut) -> TransportHeaderMut { 
        TransportHeaderMut::UDP(udp_header)
    }
}
