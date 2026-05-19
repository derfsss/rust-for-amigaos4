/*
 * Storage for the AmiSSL globals that amigaos4-sys's `extern static IAmiSSL`
 * declarations expect. amisslmaster's open call fills these in. They're not
 * provided by libauto, so any binary that touches the amissl/amisslmaster
 * wrappers needs to define them itself.
 *
 * The amissl SDK normally generates these via a magic header guarded by
 * __USE_AUTO_AMISSL; we don't go through that route, so the storage lives
 * here and gets pointed at the real interface by amisslmaster_open_ami_ssl.
 */

#include <exec/types.h>

struct Library      *AmiSSLBase           = NULL;
struct AmiSSLIFace  *IAmiSSL              = NULL;
struct Library      *AmiSSLMasterBase     = NULL;
struct AmiSSLMasterIFace *IAmiSSLMaster   = NULL;
