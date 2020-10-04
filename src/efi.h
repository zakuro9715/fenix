typedef struct EFISystemTable {
  char _buf[60];
  struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    unsigned long long _buf;
    unsigned long long (*OutputString)(
        struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This, unsigned short *String);
        unsigned long long _buf2[4];
        unsigned long long (*ClearScrenn)(
          struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This);
    } *ConOut;
} EFISystemTable;

void efi_init(
    void *ImageHandle __attribute ((unused)),
    EFISystemTable* system_table);
