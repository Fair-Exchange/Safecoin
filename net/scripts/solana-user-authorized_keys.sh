#
# Contains the public keys for users that should automatically be granted access
# to ALL testnets and datacenter nodes.
#
# To add an entry into this list:
# 1. Run: ssh-keygen -t ecdsa -N '' -f ~/.ssh/id-solana-testnet
# 2. Add an entry to SAFEANA_USERS with your username
# 3. Add an entry to SAFEANA_PUBKEYS with the contents of ~/.ssh/id-solana-testnet.pub
#
# If you need multiple keys with your username, repeatedly add your username to SAFEANA_USERS, once per key
#

SAFEANA_USERS=()
SAFEANA_PUBKEYS=()

SAFEANA_USERS+=('mvines')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBFBNwLw0i+rI312gWshojFlNw9NV7WfaKeeUsYADqOvM2o4yrO2pPw+sgW8W+/rPpVyH7zU9WVRgTME8NgFV1Vc=')

SAFEANA_USERS+=('carl')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBOk4jgcX/VWSk3j//wXeIynSQjsOt+AjYXM/XZUMa7R1Q8lfIJGK/qHLBP86CMXdpyEKJ5i37QLYOL+0VuRy0CI=')

SAFEANA_USERS+=('jack')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBEB6YLY4oCfm0e1qPswbzryw0hQEMiVDcUxOwT4bdBbui/ysKGQlVY8bO6vET1Te8EYHz5W4RuPfETbcHmw6dr4=')

SAFEANA_USERS+=('trent')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEZC/APgZTM1Y/EfNnCHr+BQN+SN4KWfpyGkwMg+nXdC trent@fry')
SAFEANA_USERS+=('trent')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDgdbzGLiv9vGo3yaJGzxO3Q2/w5TS4Km2sFGQFWGFIJ trent@farnsworth')
SAFEANA_USERS+=('trent')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHD7QmrbCqEFYGmYlHNsfbAqmJ6FRvJUKZap1TWMc7Sz trent@Trents-MacBook-Pro.local')
SAFEANA_USERS+=('trent')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIN2NCuglephBlrWSpaLkGFdrAz1aA3vYHjBVJamWBCZ3 trent@trent-build')

SAFEANA_USERS+=('dan')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBKMl07qHaMCmnvRKBCmahbBAR6GTWkR5BVe8jdzDJ7xzjXLZlf1aqfaOjt5Cu2VxvW7lUtpJQGLJJiMnWuD4Zmc= dan@Dans-MBP.local')
SAFEANA_USERS+=('dan')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBLG+2CSMwuSjX1l4ke7ScGOgmE2/ZICvJUg6re5w5znPy1gZ3YenypoBkoj3mWmavJ09OrUAELzYj3YQU9tSVh4= dan@cabbage-patch')

SAFEANA_USERS+=('greg')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG3eu2c7DZS+FE3MZmtU+nv1nn9RqW0lno0gyKpGtxT7 greg@solana.com')

SAFEANA_USERS+=('tyera')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBDSWMrqTMsML19cDKmxhfwkDfMWwpcVSYJ49cYkZYpZfTvFjV/Wdbpklo0+fp98i5AzfNYnvl0oxVpFg8A8dpYk=')

#valverde/sagan
SAFEANA_USERS+=('sakridge')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIxN1jPgVdSqNmGAjFwA1ypcnME8uM/9NjfaUZBpNdMh sakridge@valverde')
#fermi
SAFEANA_USERS+=('sakridge')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILADsMxP8ZtWxpuXjqjMcYpw6d9+4rgdYrmrMEvrLtmd sakridge@fermi.local')
SAFEANA_USERS+=('sakridge')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIF5JFfLo8rNBDV6OY08n/BWWu/AMCt6KAQ+2syeR+bvY sakridge@curie')

SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBHnXXGKZF1/qjhsoRp+7Dm124nIgUbGJPFoqlSkagZmGmsqqHlxgosxHhg6ucHerqonqBXtfdmA7QkZoKVzf/yg= buildkite-agent@dumoulin')
#ci-testnet-deployer (defunct)
SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBJnESaQpgLM2s3XLW2jvqRrvkBMDd/qGDZCjPR4X/73IwiR+hSw220JaT1JlweRrEh0rodgBTCFsWYSeMbLeGu4= buildkite-agent@ci-testnet-deployer')
#pacman (used by colo-testnet-deployer)
SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBL5bkSnIRevXXtx/sSvVtioeiLv9GLqchABi8JfMLolyv/az9mJxu77UGsxcK05ebuVQPe3PHne9isQPyrdxaE4= buildkite-agent@pacman')
#bernal
SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBJVDa1pi+Sh5N0xaBqiD+3T1c3eKT9M7Y3NIN/pCLmO9N4AH8GBVg2SeqRk4bDfPqDO6MCvSpEeOO7EBuOPVANM= buildkite-agent@bernal')
#valverde
SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBNYFFHaMi+XaHzAstLgk46kvyn2/gC/f2rCCHqbgdBqHQxyxTGZc/DlAJIqd/lQZiGhHVFRW7olnIkyQJZy5FXU= buildkite-agent@valverde')
#achilles
SAFEANA_USERS+=('buildkite-agent')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBOp9DIT0f3e/YJK3kCRXunrIAVxuy+5aOzP2jpSPDIzy/9/QAu9P0ZccHQRZTamMtEwB1g4MeafM8yFYzMf8XGU= buildkite-agent@achille')

SAFEANA_USERS+=('jstarry')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBCdpItheyXVow+4j1D4Y8Xh+dsS9GwFLRNiEYjvnonV3FqVO4hss6gmXPk2aiOAZc6QW3IXBt/YebWFNsxBW2xU= jstarry@Justin-Safecoin.local')

SAFEANA_USERS+=('ryoqun')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAsOLWUu8wbe6C5IdyB+gy1KwPCggiWv2UwhWRNOI6kV ryoqun@ubuqun')

SAFEANA_USERS+=('aeyakovenko')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEl4U9VboeYbZNOVn8SB1NM29HeI3SwqsbM22Jmw6975 aeyakovenko@valverde')

SAFEANA_USERS+=('alexander')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINHM/Rdj1UtrqPWMWjgXkjr5xFkyV0yRseM/uHxlHmxe alexander@solana.com')

SAFEANA_USERS+=('behzad')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBBBalCzbx2ObVYy4OdkKi8csr4xxutk8iVhMWqLCjeSbDgyR9p6LgnDyEKLE+B0qyn3Os85ochKdnvWVopEyPjc= behzad@solana.com')

SAFEANA_USERS+=('joe')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIP4aNf4MPb9oVGHjsneIF3iyBRMu4+J62G6hk0AptFAa joe@solana.com')

SAFEANA_USERS+=('jwash')
SAFEANA_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBHe1QgXIAip9AR8Av5KSMxyhjP3FPIBGHKpug0/bBSsO0+gcmZiXsfmIJvRlkk5rfzCBh1VXyF2ae4OzGdazWjI= jeffreywashington@JeffMBPro.local')

SAFEANA_USERS+=('dmitri')
SAFEANA_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHSYk99LYQ82tnVUav8Mu/ZrQGXOzt4esSWTbd9IV3FF dmitri@solana.com')
