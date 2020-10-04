struct EFI_SYSTEM_TABLE {
  char _buf[60];
  struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    unsigned long long _buf;
    unsigned long long (*OutputString)(
        struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This, unsigned short *String);
        unsigned long long _buf2[4];
        unsigned long long (*ClearScrenn)(
          struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This);
    } *ConOut;
};

void efi_main(
  void *ImageHandle __attribute ((unused)),
  struct EFI_SYSTEM_TABLE *SystemTable)
{
  1 / 0;
  SystemTable->ConOut->ClearScrenn(SystemTable->ConOut);
  SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Hello Worald\n");

  while(1);
}
