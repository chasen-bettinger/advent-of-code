Supply-chain breaches happen, and it looks like the kong/kubernetes-ingress-controller images are the latest to fall victim: https://lnkd.in/eUkNUSbY

# Here's what we know so far:

- A DockerHub PAT used to upload release images was compromised sometime before Dec. 23rd
- The attacker used this PAT to upload a malicious version of the 3.4.0 release image directly to DockerHub
- This image contained code to mine cryptocurrency and send results to a specific wallet
- High CPU usage was reported by a user on December 29th, and the malicious images were taken down by January 2nd
- New versions were uploaded, and the keys used to upload were revoked/rotated by the maintainers

# How should you protect against attacks like this?

As a maintainer, any key you have is a key that you can leak. Lock-down and regularly audit all systems that have access to PATs like this, or choose systems that allow OIDC-based authentication to avoid this all together. CI/CD pipelines are notoriously hard to configure securely, tools like zizmor (https://lnkd.in/eGSGrMqm) help here. Signing artifacts can help even if you can't use OIDC to publish, or users pull from mirrors out of your control.

As an end-user, pin images you receive from third-parties by digest and test/malware scan them before upgrading. Check signatures if they exist.

These are just minor protections though, the only way to completely control your destiny on attacks like these is to build all of your own artifacts directly from source in a hardened build system. SLSA provides a great framework for this hardening.

Crypto-mining is a relatively minor attack vector. This could have been much worse! For further defense-in-depth, syscall monitoring in production and blocking egress might have caught this or prevented worse attacks, but with crypto-miners you're still out the CPU cost.

# Wrapping Up

Every time you run a binary you received from someone on the internet, it's basically the same as picking up a thumb drive on a sidewalk and running it on a production server. If you're going to do this, try to only get thumb drives from people you trust, and do as much checking/defense-in-depth as you can.
