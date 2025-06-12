#include <boot/multiboot2.h>
#include <types.h>

void multiboot2_retrieve(struct multiboot_info* info_ptr, multiboot2_retrieved* buf)
{
    struct multiboot_tag* tag = info_ptr->tags;

    while (tag->type != MULTIBOOT_TAG_TYPE_END)
    {
        switch (tag->type)
        {
            case MULTIBOOT_TAG_TYPE_FRAMEBUFFER:
                buf->fb = (void*)tag;
                break;

            case MULTIBOOT_TAG_TYPE_CMDLINE:
                buf->cmdline = (void*)tag;
                break;

            case MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME:
                buf->bootloader_name = (void*)tag;
                break;
            
            case MULTIBOOT_TAG_TYPE_VBE:
                buf->vbe = (void*)tag;
                break;
                
            case MULTIBOOT_TAG_TYPE_MMAP:
                buf->mmap = (void*)tag;
                break;
        }
        
        tag = (struct multiboot_tag*)((u32)tag + ((tag->size + 7) & ~7));
    }
}