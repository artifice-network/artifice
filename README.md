<h1>Artifice Peer to Peer System</h1>
 
<h2>Application Program Interface</h2>
<p>in order to use the api to create custom applications please see api</P>
<h2>Goals</h2>
<p>The purpose of this project is to create a peer to peer secured network for IOT devices. in the long run, through its extensibility and api, this project will provide an open source, publicly available alternative to smart systems such as those manufactured by amazon, facebook, apple etc, and in so doing limit the ability to collect non-granted user information</p>
 
<h2>Long term goals</h2>
<p>Deploy artifice on top of a modified redox os, designed to provide networking capabilities through not only the NIC, but for raspberry pis the GPIO pins, and for desktops/laptops USB-C and PCI</p>

<h2>Building and installing</h2>
<h4>Building</h4>
<p>In order to build this project simple use git and cargo</p>
<h6>clone the repo</h6>
<code>git clone https://github.com/cardinalcat/artifice</code>
<h6>build with cargo</h6>
<code>cargo build</code>
<h4>Installation</h4>
<p>Installation is currently not supported due to the abstract nature of the installation configuration, however when available the code will be posted to crates.io so that cargo install artifice will work</p>
 
<h2>Implementation</h2>
<h4>Networking</h4>
<p>the networking uses RSA encryption to pass data to peers on the network, and preserves limited information about peers in the network<br>the peers are identified in a two factor authentication by using pre-shared keys as well as a pair key that is unique to each pair of peers<br> the reason for implementing a pair key authentication is in case the public key is misused to attempt to fake the identity of a different peer</P>
<h4>Local protections</h4>
<p>this project aims to maintain the security of a peer even if the entire system has been compromised by a virus, in an effort to accomplish this goal, all information about peers and configuration is encrypted using a Sha256 hash generated using a user defined password that isn't stored anywhere<br> thus if the password is lost the establishment of the local hosts peers will need to be redone</p>
 
<h4>Permissions System</h4>
<p>This project aims to provide a means for extensibility, or the ability to add features that work on this network, as such it provides (or will provide) an api for access to the network. this api is structured in such a way that an application can gain permissions to perform operations both locally and remotely, if such permission is granted. the implementation of the permission system again uses encrypted files to store a whitelist of applications, and peers that are allow access to a given permission, it is done in a way that permissions have peers who are granted them, rather then having permissions granted to a peer/api</p>
 
<p>the implementation of this permission system for operations outside of the artifice network, such as local file access will be implemented using docker images, so the extended application (application that uses artifice api) has limited to no access to the host system, and thus is less able to cause damage to the system, or compromise other peers. In building this project is my aim to allow the api to remain safe even when the host system may be compromised to prevent other peers from easily being compromised</p>
 
<h2>Terms of Service</h2>
<h4>Desire</h4>
The simple effort of this project is to provide a new safer implementation of computing technology. as such no user will be subject to dataharvisting. This right is also guaranteed by the use of a peer to peer networking to offer proxy capabilities.
 
<h4>User Responsibility</h4>
<p>it is encourage to mess around with the source code of this project, and if at all possible to find vulnerabilities, however the condition associated with such action is simply that it is not allowed in any form that may infringe on the rights of other individuals, bloatware, ransomware, and malware being spread through this networking will be fought, and is absolutely not tolerated, unless it is for ben testing and only performed on devices owned by the individual performing such actions.</p>
 
<h4>Special Situations</h4>
<p>This page is being written, and this api is being developed during a pandemic, a situation that could easily be avoided had mandatory automated contact tracing been implemented in an anonymous manner. In this kind of a situation these protections might be violated, though seeing as users are not tracked at all such an occurrence is currently impossible, however should there come a time when a peer is associated with a real life person this type of contact tracing may be implemented.</p>
 
<h4>Situations in which privacy may be infringed</h4>
<ul>
<li>global pandemic</li>
<li>threat of world war, or genicide</li>
<li>child endangerment</li>
</ul>
<h4>Situations in which privacy will not be infringed</h4>
<ul>
<li>piracy</li>
<li>Illegal drug exchange/usage</li>
<li>public opinion hacking</il>
<li>Advertising</li>
</ul>

