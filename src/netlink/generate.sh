#!/usr/bin/env sh

bindgen \
  --whitelist-type sockaddr_nl \
  --whitelist-type nlmsghdr \
  --whitelist-type nlmsgerr \
  --whitelist-type nlmsgerr_attrs \
  --whitelist-type nl_pktinfo \
  --whitelist-type nl_mmap_req \
  --whitelist-type nl_mmap_hdr \
  --whitelist-type nl_mmap_status \
  --whitelist-type nlattr \
  --whitelist-type nla_bitfield32 \
  --whitelist-var "NETLINK_.+" \
  --whitelist-var "NLM_F_.+" \
  --whitelist-var "NLMSG_.+" \
  --whitelist-var "NL_MMAP_.+" \
  --whitelist-var "NLA_.+" \
  /usr/include/linux/netlink.h