/*
 * Storage for the z.library globals that amigaos4-sys's `extern static
 * IZ` declaration expects. z.library is NOT in libauto's set; any
 * binary that uses the z wrappers has to open the library and bind
 * its interface itself. We exec_open_library + exec_get_interface from
 * Rust and stash the results in these globals before any wrapper call.
 */

#include <exec/types.h>

struct Library *ZBase = NULL;
struct ZIFace  *IZ    = NULL;
