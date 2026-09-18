use core::marker::PhantomData;

use crate::core::{Command, Control, OpCode, OpCodeDef, ReadCommand};

pub mod standard_inquiry;

mod private {
    pub trait VpdPageSeal {}
    pub trait InquirySeal {
        const PAGE_OP_CODE: u8;
        const EVPD: bool;
        const CMD_DT: bool;
    }
}

pub trait InquiryType: private::InquirySeal {}

pub trait VpdPage: private::VpdPageSeal {
    const PAGE_CODE: u8;
}

impl private::InquirySeal for standard_inquiry::StandardInquiry {
    const PAGE_OP_CODE: u8 = 0x00;
    const CMD_DT: bool = false;
    const EVPD: bool = false;
}
impl InquiryType for standard_inquiry::StandardInquiry {}

pub struct VpdInquiry<T: VpdPage>(PhantomData<T>);

impl<T: VpdPage> private::InquirySeal for VpdInquiry<T> {
    const PAGE_OP_CODE: u8 = T::PAGE_CODE;
    const CMD_DT: bool = false;
    const EVPD: bool = true;
}
impl<T: VpdPage> InquiryType for VpdInquiry<T> {}

pub struct OpCodeInquiry<T: OpCodeDef>(PhantomData<T>);

impl<T: OpCodeDef> private::InquirySeal for OpCodeInquiry<T> {
    const PAGE_OP_CODE: u8 = T::OP_CODE;
    const CMD_DT: bool = true;
    const EVPD: bool = false;
}
impl<T: OpCodeDef> InquiryType for OpCodeInquiry<T> {}

pub struct Inquiry<T: InquiryType> {
    _page_code_marker: PhantomData<T>,
    allocation_length: u16,
    control: Control,
}

impl<T: InquiryType> Inquiry<T> {
    pub fn new(allocation_length: u16, control: Control) -> Self {
        Self {
            _page_code_marker: PhantomData,
            allocation_length,
            control,
        }
    }
}

type InquiryOpCode = OpCode<0x12>;

impl<T: InquiryType> Command<InquiryOpCode> for Inquiry<T> {
    fn as_cdb(&self) -> <InquiryOpCode as OpCodeDef>::Cdb {
        [
            InquiryOpCode::OP_CODE,
            (u8::from(T::CMD_DT) << 1) | u8::from(T::EVPD),
            T::PAGE_OP_CODE,
            (self.allocation_length >> 8) as u8,
            self.allocation_length as u8,
            self.control.into(),
        ]
    }
}

impl<T: VpdPage> ReadCommand<InquiryOpCode> for Inquiry<VpdInquiry<T>> {
    type Len = u16;
    type Response<'a> = T;
    type Error = (); // TODO

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    // TODO
    fn parse<'a>(&self, _buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        todo!()
    }
}

impl<T: OpCodeDef> ReadCommand<InquiryOpCode> for Inquiry<OpCodeInquiry<T>> {
    type Len = u16;
    type Response<'a> = T;
    type Error = (); // TODO

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    // TODO
    fn parse<'a>(&self, _buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        todo!()
    }
}
