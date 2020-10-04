# include "efi.h"

void efi_init(
  void *ImageHandle __attribute ((unused)),
  SystemTable* system_table)
{
  system_table->boot_service->SetWatchdotTimer(0, 0, 0, nullptr);
  system_table_->boot_services->LocateProtocol(
      &kGraphicsOutputProtocolGUID, nullptr,
      (void**)&graphics_output_protocol_);
  );
  LoadedImageProtocol* loaded_image_protocol;
}
