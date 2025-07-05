[bits 32]

section .text
global memset32
memset32:
  mov edi, [esp + 4]
  mov eax, [esp + 8]
  mov ecx, [esp + 12]
  rep stosd
  ret

