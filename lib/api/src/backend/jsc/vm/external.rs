use wasmer_types::RawValue;

use crate::{AsStoreMut, Extern, VMExternToExtern};

use super::{function::VMFunction, global::VMGlobal, memory::VMMemory, table::VMTable};

/// The value of an export passed from one instance to another in the `jsc` VM.
pub enum VMExtern {
    /// A function export value.
    Function(VMFunction),

    /// A table export value.
    Table(VMTable),

    /// A memory export value.
    Memory(VMMemory),

    /// A global export value.
    Global(VMGlobal),
}

impl VMExternToExtern for VMExtern {
    fn to_extern(self, store: &mut impl AsStoreMut) -> Extern {
        match self {
            Self::Function(f) => Extern::Function(crate::Function::from_vm_extern(
                store,
                crate::vm::VMExternFunction::Jsc(f),
            )),
            Self::Memory(m) => Extern::Memory(crate::Memory::from_vm_extern(
                store,
                crate::vm::VMExternMemory::Jsc(m),
            )),
            Self::Global(g) => Extern::Global(crate::Global::from_vm_extern(
                store,
                crate::vm::VMExternGlobal::Jsc(g),
            )),
            Self::Table(t) => Extern::Table(crate::Table::from_vm_extern(
                store,
                crate::vm::VMExternTable::Jsc(t),
            )),
        }
    }
}

/// A reference to an external value in the `js` VM.
pub struct VMExternRef;

impl VMExternRef {
    /// Converts the `VMExternRef` into a `RawValue`.
    pub fn into_raw(self) -> RawValue {
        unimplemented!();
    }

    /// Extracts a `VMExternRef` from a `RawValue`.
    ///
    /// # Safety
    /// `raw` must be a valid `VMExternRef` instance.
    pub unsafe fn from_raw(_raw: RawValue) -> Option<Self> {
        unimplemented!();
    }
}
