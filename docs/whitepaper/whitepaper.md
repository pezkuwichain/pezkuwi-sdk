# PezkuwiChain: A Sovereign Blockchain Infrastructure for a Digital Nation
**Technical Whitepaper v3.0**
**November 2025**
**Prepared by:** Kurdistan Tech Ministry & PezkuwiChain Katılımcıları

---

## Abstract
PezkuwiChain, sosyal itibarı blockchain güvenliğine entegre eden yeni bir konsensüs mekanizması olan Trust-enhanced Nominated Proof-of-Stake (T-NPoS) sistemini sunar. Polkadot SDK üzerine inşa edilen ve gelişmiş çift tokenlı bir ekonomi (HEZ/PEZ) içeren PezkuwiChain, Kürt ulusu için kimlik, yönetişim, eğitim ve hazine yönetimi için özel palletler içeren egemen bir dijital altyapı sağlar. Bu whitepaper, dünyanın ilk güven artırılmış Katman-1 blockchain'i için teknik mimariyi, ekonomik modeli, güvenlik çerçevesini ve stratejik yol haritasını sunmaktadır.

---

## Table of Contents
1.  Yönetici Abstracti
2.  Introduction
3.  The Problem
4.  The Solution: PezkuwiChain Architecture (Multi-Chain Updated)
5.  Dual-Token Economic Model
6.  Core Features & Custom Pallets
7.  Technical Specifications
8.  Network Architecture
9.  Governance Model
10. Security and Auditing
11. Roadmap & Development Phases (Updated)
12. Use Cases & Applications
13. Team & Contributors
14. Ecosystem & Partnerships
15. Legal & Compliance
16. Conclusion
17. References
18. Contact & Resources
19. Appendix A: Glossary
20. Appendix B: Developer Resources

---

## 1. Yönetici Abstracti

PezkuwiChain, Kürt bölgesinin ve küresel diasporasının dijital altyapı ihtiyaçlarını karşılamak üzere titizlikle tasarlanmış egemen bir Katman-1 blockchain ağıdır. Güçlü ve sahada test edilmiş Polkadot SDK üzerine inşa edilen PezkuwiChain, yenilikçi Trust-enhanced Nominated Proof-of-Stake (TNPoS) konsensüs mekanizmasını, sofistike çift tokenlı ekonomik modeli ve yönetişim, kimlik ve eğitim için özel olarak oluşturulmuş kapsamlı bir pallet paketini sunar.

Projenin vizyonu, merkeziyetsiz teknoloji aracılığıyla Kürt ulusunu güçlendirmek, finansal katılımı, dijital kimliği ve sosyal güveni temel konsensüs katmanına entegre eden şeffaf, topluluk odaklı bir ekosistemi teşvik etmektir. Bu whitepaper, PezkuwiChain mimarisine, çığır açan TNPoS konsensüsüne, teknik özelliklerine ve stratejik yol haritasına kapsamlı bir genel bakış sunmaktadır.

**Temel İnovasyonlar:**
*   **TNPoS Konsensüsü:** Dünyanın sosyal itibarı doğrudan konsensüse entegre eden ilk güven artırılmış PoS mekanizması.
*   **Çift Tokenlı Ekonomi:** HEZ (enflasyonist) + PEZ (sabit 5 Milyar arz).
*   **Çoklu Zincir Mimarisi:** Relay Chain, Asset Hub ve People Chain olarak uzmanlaşmış Parachain'ler.
*   **Parliamentary NFT Sistemi:** Ödül mekanizmalarıyla 201 yönetişim NFT'si.
*   **Özel Palletler:** Dijital egemenlik için 12 uzmanlaşmış modül (`identity-kyc`, `welati`, `perwerde`, `pez-treasury`, `pez-rewards`, `validator-pool`, `staking-score`, `trust`, `referral`, `tiki`, `pallet-presale`, `pallet-token-wrapper`).
*   **Polkadot SDK Temeli:** Kanıtlanmış güvenlik ve birlikte çalışabilirlik.

---

## 2. Introduction

Blockchain teknolojisinin ortaya çıkışı, merkeziyetsiz, şeffaf ve güvenli dijital altyapılar oluşturmak için eşi benzeri görülmemiş fırsatlar sunmuştur. Ancak, mevcut blockchain çözümlerinin çoğu, belirli kültürel, ekonomik ve yönetişim ihtiyaçlarını ele almakta çoğu zaman yetersiz kalan genel amaçlı platformlar olarak tasarlanmıştır.

PezkuwiChain, Kürt ulusu için özel bir dijital devlet yaratma vizyonuyla ortaya çıkmış, blockchain'in gücünden yararlanarak köklü sorunları ele almayı ve müreffeh bir dijital gelecek için temel oluşturmayı amaçlamaktadır. PezkuwiChain sadece bir kripto para birimi değil; dijital egemenlik için araçlar sağlamak üzere tasarlanmış kapsamlı bir ekosistemdir.

Misy, Kürt halkına finansal hizmetler, dijital kimlik, demokratik yönetişim ve eğitim için güvenli ve merkeziyetsiz bir platform sağlayarak kamu hizmeti sunmaktır. Bu belge, PezkuwiChain'in teknik ve ekonomik mimarisini, benimsenmesini ve uzun vadeli başarısını sağlayacak yenilikçi çözümlerini ve stratejik vizyonunu detaylandırmaktadır.

---

## 3. The Problem

Geleneksel finansal ve idari sistemler genellikle önemli giriş engelleri sunar, şeffaflıktan yoksundur ve küresel olarak dağılmış, ancak kültürel olarak birleşik ulusların benzersiz ihtiyaçlarına kötü uyum sağlar. On milyonlarca Kürt halkı, egemen bir dijital altyapının ele alabileceği farklı zorluklarla karşı karşıyadır:

**Finansal Dışlanma:** Nüfusun önemli bir kısmı modern bankacılık ve finansal hizmetlere erişememekte, ekonomik büyümeyi ve bireysel refahı engellemektedir.

**Dijital Egemenlik Eksikliği:** Birleşik, egemen bir dijital kimlik sisteminin olmaması, vatandaş katılımını, hizmetlere erişimi ve sınırlar arası yasal tanınmayı zorlaştırmaktadır.

**Yönetişim Açıkları:** Merkezi yönetişim modelleri opak olabilir ve geniş, demokratik katılım için mekanizmalardan yoksun olabilir, halkın kolektif iradesini yakalamakta başarısız olabilir.

**Ekonomik Dalgalanma:** Ulusal ekonomiler genellikle dış para birimlerinin ve jeopolitik baskıların oynaklığına karşı hassastır. Yerel bir dijital para birimi daha istikrarlı ve kontrollü bir ekonomik ortam sağlayabilir.

**Blockchain'deki Güven Açığı:** Mevcut blockchain konsensüs mekanizmaları, dirençli ve topluluk odaklı ağlar oluşturmak için temel olan sosyal güveni ve itibarı birleştirmede başarısız olmaktadır. Saf ekonomik teşvikler, merkezileşmeye ve yanlış hizalanmış davranışlara yol açabilir.

### 3.1. Rekabet Ortamı

PezkuwiChain'in benzersiz değer önerisini anlamak için, önde gelen Katman-1 blockchain platformlarıyla karşılaştırmasını sunuyoruz:

| Özellik                | Ethereum          | Polkadot           | Cardano              | PezkuwiChain (Güncel)  | 
| :--------------------- | :--------------   | :--------------    | :----------------    | :--------------------  | 
| Konsensüs              | PoS               | NPoS               | Ouroboros PoS        | TNPoS                  | 
| Yönetişim              | DAO (Zincir dışı) | OpenGov            | Voltaire             | Welati (Zincir içi)    | 
| Dijital Kimlik         | ❌ Üçüncü taraf   | ❌ Üçüncü taraf   | ⚠ Atala PRISM        | ✅ identity-kyc       | 
| Eğitim Platformu       | ❌ Yok            | ❌ Yok            | ❌ Yok               | ✅ perwerde           | 
| Güven Katmanı          | ❌ Yok            | ❌ Yok            | ❌ Yok               | ✅ pallet-trust       | 
| Hazine Modeli          | DAO tabanlı       | %15 enflasyon      | Protokol ücretleri   | %20.25 PEZ             | 
| Kültürel Odak          | Genel             | Genel              | Genel                | Kürt Ulusu + ( kulturel uluslar + stateless nations + genel )         | 
| Parachain Hazırlığı    | ❌ Yok            | ✅ Evet           | ❌ Yok               | ✅ Evet (Cumulus)     | 

**Temel Ayırt Ediciler:**
1.  **Güven Entegrasyonu:** PezkuwiChain, sosyal itibarı doğrudan konsensüse entegre eden tek blockchaindir.
2.  **Egemen Kimlik:** Dahili KYC/AML uyumlu kimlik sistemi (`identity-kyc` paleti).
3.  **Eğitim Altyapısı:** Yerel eğitim ve sertifikasyon platformu (`perwerde` paleti).
4.  **Kültürel Uyum:** Özellikle Kürt dijital egemenliği için tasarlanmıştır.
5.  **Çift Token Yeniliği:** Ekonomik güvenlik (HEZ) ve yönetişim (PEZ) için ayrı tokenler.

---

## 4. The Solution: PezkuwiChain Architecture (Multi-Chain Updated)

PezkuwiChain, bu zorluklara kapsamlı bir çözüm olarak mimarisi tasarlanmıştır ve Kürt ulusu için güvenli, merkeziyetsiz ve egemen bir dijital omurga sağlar. Yeni nesil blokzincirler oluşturmak için en son teknoloji çerçevesi olan Polkadot SDK kullanılarak inşa edilmiştir.

### 4.1. Temel: Polkadot SDK ve Substrate
Polkadot SDK seçimi, PezkuwiChain'e sağlam, modüler ve geleceğe dönük bir temel sağlar. Bu mimarinin çekirdeği, blokzincirinin çekirdek mantığını (Runtime) istemci tarafı işlevlerinden (Client) ayıran Substrate'dir.

**Runtime (Durum Geçiş Fonksiyonu):** İş mantığını içeren blokzincirinin kalbidir. Çatallanmasız, zincir içi yükseltmeler için WebAssembly (Wasm) olarak derlenir. Bu, ağın yıkıcı sert çatallar olmadan evrimleşebileceği ve adapte olabileceği anlamına gelir.

**Client:** Ağ iletişimini (Libp2p aracılığıyla), konsensüsü (BABE ve GRANDPA) ve işlem yönetimini yöneten yerel ikilidir. Wasm runtime'ını yürüten ana bilgisayar olarak işlev görür.

Modüler Varlıkların Çalışma Zamanı Agregasyonu için Çerçeve (FRAME) tarafından kolaylaştırılan bu modüler tasarım, önceden oluşturulmuş modüllerin (palletler) sorunsuz entegrasyonuna ve PezkuwiChain'in belirli ihtiyaçlarına göre uyarlanmış özel mantığın geliştirilmesine olanak tanır.

### 4.2. Konsensüs İnovasyonu: Güven Geliştirilmiş Nominated Proof-of-Stake (TNPoS)

PezkuwiChain, bir Güven Sistemini doğrudan validatör seçimi ve ödül dağıtım sürecine entegre ederek geleneksel Nominated Proof-of-Stake (NPoS) konsensüs mekanizmasına çığır açan bir geliştirme sunar. TNPoS (Güven Geliştirilmiş Nominated Proof-of-Stake) olarak adlandırılan bu yeni yaklaşım, NPoS'un ekonomik güvenliğini özel `pallet-trust` tarafından sağlanan sosyal itibar katmanıyla birleştirir.

### Figure 1: TNPoS Konsensüs Akışı - Güven Puanlarının Validatör Seçimine ve Ödüllerine Entegrasyonu
(Görselin içeriği markdown'a aktarılamaz, ancak akış aşağıdaki gibidir: Trust Layer -> Generate -> Trust Score System (pallet-trust) -> Weight -> TNPoS Core (Validator Selection Stake + Trust Score) -> Elect -> Validator Pool Active Set -> Produce Blocks. Ayrıca Rewards Distribution kısmından Trust-Based Rewards (pallet-pez-rewards) kısmı ile ilişkilidir.)

### 4.2.1. TNPoS'un Temel Bileşenleri
**Geleneksel NPoS Temeli:** TNPoS, Polkadot'un kanıtlanmış NPoS modeline dayanır; burada Validatörler blok üretmekten ve zinciri kesinleştirmekten sorumluyken, Adaylar güvenilen Validatörleri desteklemek ve seçmek için tokenlerini stake ederler. Bu sistem, ağ güvenliğine katılımı demokratikleştirir ve yüksek düzeyde ekonomik güvenlik sağlar.

**Güven Puanı Entegrasyonu:** `pallet-trust`, ağ katılımcılarının davranışları, katkıları ve ekosistem içindeki etkileşimleri temelinde güven puanları oluşturmalarına olanak tanıyan bir sosyal güven ve itibar sistemi sunar. Bu güven puanları sadece kozmetik değildir; `pallet-pez-rewards` mekanizması aracılığıyla konsensüs katmanına entegre edilmiştir.

**Güven Ağırlıklı Validatör Seçimi:** TNPoS'ta validatör seçimi yalnızca stake miktarına dayanmaz, aynı zamanda hem Validatörlerin hem de Adaylarının güven puanlarından da etkilenir. Bu, daha yüksek topluluk güvenine ve kanıtlanmış geçmişe sahip validatörlerin seçilme olasılığının daha yüksek olmasını sağlayarak ağın genel güvenilirliğini ve güvenliğini artırır.

**Güvene Dayalı Ödül Dağıtımı:** `pallet-pez-rewards`, ödülleri hesaplamak ve dağıtmak için güven puanlarını kullanır. Daha yüksek güven puanlarına sahip Validatörler ve Adaylar orantılı olarak daha yüksek ödüller alırlar, bu da iyi davranışları, uzun vadeli taahhüdü ve ağın yönetişim ve güvenliğine aktif katılımı teşvik eden pozitif bir geri bildirim döngüsü yaratır.

### 4.2.2. Blok Üretimi ve Kesinlik
PezkuwiChain, blok üretimi ve kesinlik için hibrit bir konsensüs mekanizması kullanır:

**BABE (Blind Assignment for Blockchain Extension):** Blok üretiminden sorumludur. BABE, validatörlere blok oluşturma haklarını rastgele atayan, sansür direncini sağlayan ve yaklaşık 6 saniyelik tutarlı bir blok süresi sağlayan slot tabanlı bir protokoldür.

**GRANDPA (GHOST-based Recursive ANcestor Deriving Prefix Agreement):** Zincir için kesinlik aygıtı olarak hizmet eder. GRANDPA, çok sayıda validatörün blokların kesinliği konusunda hem hızlı hem de kanıtlanabilir şekilde güvenli bir şekilde anlaşmasına olanak tanır. Bir blok, validatörlerin süper çoğunluğu (2/3+) tarafından onaylandıktan sonra kesin kabul edilir.

### 4.2.3. Blockchain Teknolojisine İnovasyon ve Katkı
TNPoS, blockchain konsensüs tasarımında önemli bir yeniliği temsil eder. Sosyal güveni ekonomik güvenlik modeline entegre ederek, PezkuwiChain daha dirençli, topluluk odaklı ve sürdürülebilir bir ağ oluşturur.

**Geleneksel Konsensüs Mekanizmaları ile Karşılaştırma:**

| Özellik                | PoW (Bitcoin)        | PoS (Ethereum)    | NPoS (Polkadot)           | TNPoS (PezkuwiChain) | 
| :--------------------- | :---------------     | :-------------    | :----------------         | :------------------- | 
| Enerji Verimliliği     | ❌ Çok Düşük         | ✅ Yüksek        | ✅ Yüksek                 | ✅ Çok Yüksek       | 
| Validatör Seçimi       | Madencilik Gücü      | Stake Miktarı     | Stake + Adaylık           | Stake + Güven Puanı  | 
| Ödül Dağıtımı          | Madencilik Ödülleri  | Stake Ağırlıklı   | Eşit Temel + Era Puanları | Güvene Dayalı        | 
| Merkezileşme Riski     | Orta-Yüksek          | Yüksek            | Düşük-Orta                | Çok Düşük            | 
| Sosyal Katman          | ❌ Yok               | ❌ Yok           | ❌ Yok                    | ✅ Güven Katmanı    | 
| Yönetişim Entegrasyonu | ❌ Zincir dışı       | ⚠ Kısmi          | ✅ Tam (OpenGov)          | ✅ Tam (Welati)     | 
| Slashing Mekanizması   | Yok                  | ✅ Evet          | ✅ Evet                    | ✅ Evet             | 
| Sybil Direnci          | Yüksek Maliyet       | Yüksek Maliyet    | Yüksek Maliyet             | Çok Yüksek Maliyet  | 

Bu yaklaşım, mevcut PoS sistemlerindeki kritik bir boşluğu ele alır; burada yalnızca ekonomik teşvikler bazen merkezileşmeye veya yanlış hizalanmış davranışlara yol açabilir. TNPoS, ağın yalnızca sermaye ile değil, aynı zamanda itibar ve topluluk güveni ile de güvence altına alınmasını sağlayarak, egemen bir dijital devlet için benzersiz bir şekilde uygundur.

### 4.3. Sistem Mimarisi

### Figure 2: PezkuwiChain Sistem Mimarisi - Kullanıcılardan Çalışma Zamanı Palletlerine Katmanlı Görünüm
(Görselin içeriği markdown'a aktarılamaz, ancak genel mimari katmanlıdır ve şu bileşenleri içerir: User Layer, Interface Layer, Client Layer, Runtime Layer (Custom Pallets ve FRAME System Pallets), Integration Layer (Cumulus ve XCM).)

Mimari katmanlı bir yaklaşımı takip eder:
1.  **Kullanıcı Katmanı:** Son kullanıcılar, validatörler, geliştiriciler ve hükümet birimleri.
2.  **Arayüz Katmanı:** Web UI, CLI araçları, RPC/WebSocket API'leri ve SDK'lar.
3.  **İstemci Katmanı:** Ağ iletişimi, konsensüs ve veritabanı ile Substrate istemcisi.
4.  **Runtime Katmanı:** WebAssembly'de özel palletler ve FRAME sistem palletleri.
5.  **Entegrasyon Katmanı:** Parachain desteği için Cumulus ve zincirler arası iletişim için XCM.

---

## 5. Dual-Token Economic Model

PezkuwiChain, hem kamu hem de hükümet işlevlerine hizmet eden dengeli ve sürdürülebilir bir ekosistem oluşturmak için yenilikçi bir çift tokenlı ekonomik model sunar. İki yerel token, HEZ ve PEZ, farklı amaçlar ve parasal politikalarla tasarlanmıştır.

### Figure 3: Çift Tokenlı Ekonomi Akışı - HEZ ve PEZ token dağılımı ve faydası
(Görselin içeriği markdown'a aktarılamaz, ancak HEZ'in enflasyonist, PEZ'in sabit arzlı ve sentetik yarılanmalı olduğunu gösterir. Dağılım oranları ve kullanım alanları özetlenir.)

### 5.1. HEZ: Halkın Para Birimi

HEZ, PezkuwiChain ağının yerel, enflasyonist kripto para birimidir ve Polkadot ekosistemindeki standart DOT token modeline tamamen uyumludur. Birincil rolü, stake etme yoluyla ağı güvence altına almak ve işlem ücretleri ile günlük ekonomik faaliyetler için birincil değişim aracı olarak hizmet etmektir. Enflasyonist model, ağ katılımını sürekli teşvik etmek ve zinciri güvence altına almak için Validatörleri ve Adayları ödüllendirmek üzere tasarlanmıştır. Enflasyon oranı, optimum ağ güvenliğini sağlamak için stake oranına göre dinamik olarak ayarlanır.

### 5.1.1. HEZ Token Ekonomisi
HEZ, Polkadot'un DOT tokeni tarafından oluşturulan ve başlangıcından bu yana Polkadot ağını başarıyla güvence altına alan kanıtlanmış ekonomik modeli takip eder. HEZ token ekonomisinin temel özellikleri şunlardır:

**Enflasyon Modeli:** HEZ, Polkadot'un Kasım 2024 sonrası tokenomiklerine benzer sabit bir yıllık emisyon modeli kullanır. Ağ, yıllık olarak önceden belirlenmiş miktarda HEZ tokeni basar ve bu tokenler, dikkatle tasarlanmış bir tahsis stratejisine göre ağ katılımcılarına dağıtılır. Bu doğrusal ihraç modeli, güçlü güvenlik teşviklerini korurken öngörülebilir token arzı büyümesini sağlar.

**Staking Ödülleri Dağıtımı:** Yıllık HEZ emisyonu, ağ güvenliği ve ekosistem sürdürülebilirliğine öncelik verilerek tahsis edilir. Polkadot standardını takiben, yıllık enflasyonun yaklaşık %85'i stake ödüllerine yönlendirilir ve Validatörler ile Adayları arasında dağıtılır. Kalan %15, ekosistem geliştirme, topluluk teklifleri ve kamu mallarını finanse etmek için ağ hazinesine akar. Bu dağıtım, yeni basılan tokenlerin çoğunluğunun doğrudan ağ güvenliğini teşvik etmesini sağlarken, uzun vadeli büyüme için sürdürülebilir bir finansman mekanizmasını sürdürür.

**Eşit Validatör Ödülleri:** NPoS sisteminin en yenilikçi yönlerinden biri, PezkuwiChain'in TNPoS'u tarafından miras alınan, tüm aktif Validatörlerin toplam stake desteğine bakılmaksızın eşit temel ödüller almasıdır. Bu tasarım seçimi kasıtlıdır ve birkaç ağır stake edilmiş validatör arasında güç merkezileşmesini önlemeye hizmet eder. Bunun yerine, daha dağıtılmış bir validatör setini teşvik eder ve daha küçük validatörlerin daha büyük olanlarla etkili bir şekilde rekabet etmelerine olanak tanır. Ancak, bu eşit dağıtım çerçevesi içinde, Validatörler zincir içi faaliyetleri ve performanslarına dayalı olarak ek era puanları kazanabilirler, bu da aktif ve güvenilir validatörleri ödüllendiren liyakate dayalı bir bonus sistemi oluşturur.

**Dinamik Enflasyon Mekanizması:** Temel emisyon sabit olsa da, toplam arz büyüdükçe (toplam arzın yüzdesi olarak) etkili enflasyon oranı doğal olarak azalır. Bu, güvenlik teşvikleri ihtiyacını uzun vadeli token değerinin korunmasıyla dengeleyen, kademeli olarak deflasyonist bir baskı yaratır. Stake ödülleri, ağda ideal bir stake oranını (genellikle toplam arzın %50-75'i) sürdürmek için kalibre edilir. Gerçek stake oranı bu hedefin altına düşerse, ödüller dolaşımdaki arza göre daha çekici hale gelir ve daha fazla katılımcıyı stake etmeye teşvik eder. Tersine, stake etme hedef oranını aşarsa, göreceli getiriler azalır ve bazı katılımcıları stake'i bırakmaya ve tokenlerini ekosistem içinde başka amaçlar için kullanmaya teşvik eder.

**Unbonding Süresi ve Güvenlik:** Ağ güvenliğini sağlamak ve "nothing at stake" saldırılarını önlemek için HEZ, yaklaşık 28 günlük bir unbonding süresi (Polkadot standardını takiben) uygular. Bu süre zarfında, unstake edilmiş tokenler kilitli kalır ve ödül kazanmaz. Bu mekanizma, ağın kötü niyetli davranışları tespit etmesine ve cezalandırmasına olanak tanıyan kritik bir güvenlik tamponu sağlar, çünkü validatörler ve adaylar geçmiş eralardaki eylemlerinden sorumlu kalır.

**Yanlış Davranış İçin Slashing:** HEZ staking sisteminin güvenliği, validatörlerin (ve adaylarının) kötü niyetli veya ihmalkar davrandığının tespit edilmesi halinde stake edilmiş tokenlerinin bir kısmını kaybedebilecekleri bir mekanizma olan slashing aracılığıyla uygulanır. Bu, kötü davranışlar için güçlü ekonomik caydırıcılar yaratır ve tüm katılımcıların "risk altında" olmasını sağlar. Slashing'in şiddeti, suçun doğasına ve ölçeğine bağlı olarak, kasıtsız kesinti süreleri için küçük cezalardan kanıtlanabilir kötü niyetli saldırılar için ağır slashing'e kadar değişir.

**Token Faydası:** Stake etmenin ötesinde, HEZ tüm ağ işlemleri için temel fayda tokenı olarak hizmet eder. İşlem ücretlerini ödemek, akıllı sözleşmeleri yürütmek, yönetişime (PEZ ile birlikte) katılmak ve ağ hizmetlerine erişmek için kullanılır. Bu çok yönlü fayda, HEZ tokenlerine tutarlı talep sağlar ve ağın uzun vadeli sürdürülebilirliğini destekleyen sağlam bir token ekonomisi yaratır.

### 5.2. PEZ: Yönetişim ve Ödül Tokenı

PEZ, yönetişim, hazine finansmanı ve ödül dağıtımı için tasarlanmış toplam 5 milyar birimlik sabit arzlı bir tokendir. PEZ, `pallet-assets` çerçevesi kullanılarak basılır ve Bitcoin'in yarılanma felsefesini yansıtran, ancak yönetişim odaklı bir tokene uyarlanmış, her 48 ayda (4 yılda) bir gerçekleşen sofistike bir dağıtım mekanizması ile sentetik bir yarılanma programı içerir.

### 5.2.1. PEZ Token Dağıtımı (Zincir İçi Uygulamaya Göre)
PEZ token dağıtımı, `pallet-pez-treasury` içine hardcode edilmiştir ve dikkatle tasarlanmış bir tahsis stratejisini yansıtır:

| Tahsis Kategorisi | Miktar (PEZ)      | Yüzde   | Amaç                                         | 
| :---------------- | :---------------- | :------- | :------------------------------------------- | 
| Hazine            | 1,012,500,000     | %20.25   | Topluluk projelerini ve kamu mallarını finanse etmek için zincir içi hazine. | 
| Ön Satış          | 93,750,000        | %1.875   | Erken destekçilere ve üyelere ilk dağıtım. | 
| Kurucu            | 93,750,000        | %1.875   | Kurucu ekip için tahsis, hak edişe tabi. | 
| Ödüller & Ekosistem | ~3,800,000,000    | ~%76     | Staking ödülleri, teşvik havuzu, Parlamenter NFT ödülleri ve ekosistem için tahsis edilmiştir. | 

**Toplam Arz: 5,000,000,000 PEZ**

**Teknik Sabitler (`pallet-pez-treasury`'den):**
```rust
pub const TOTAL_SUPPLY: u128 = 5_000_000_000 * 1_000_000_000_000; // 12 decimals
pub const TREASURY_ALLOCATION: u128 = 1_012_500_000 * 1_000_000_000_000;
pub const PRESALE_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000;
pub const FOUNDER_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000;
```

### 5.2.2. Sentetik Yarılanma Mekanizması
PEZ tokeni, aşağıdaki parametrelerle sentetik bir yarılanma mekanizması kullanır:

**Teknik Sabitler (`pallet-pez-treasury`'den):**
```rust
pub const HALVING_PERIOD_MONTHS: u32 = 48; // 4 years
pub const BLOCKS_PER_MONTH: u32 = 432_000; // ~30 days
pub const HALVING_PERIOD_BLOCKS: u32 = 20_736_000; // 48 months in blocks
```

### Figure 4: PEZ Token Yarılanma Programı - 20 Yıl Boyunca 48 Aylık Döngüler
(Görselin içeriği markdown'a aktarılamaz, ancak PEZ token emisyon oranlarının 48 aylık döngülerle nasıl azaldığını gösterir.)

Bu yarılanma mekanizması, ödüllerden ve ekosistem tahsisinden PEZ tokenlerinin serbest bırakılmasını kontrol ederek, zaman içinde öngörülebilir ve deflasyonist bir dağıtım sağlar.

### 5.3. Ekonomik Simülasyonlar ve Tahminler

### 5.3.1. HEZ Enflasyon Tahminleri
Polkadot'un 120M DOT modeline benzer sabit bir yıllık emisyon varsayılarak (PezkuwiChain için uygun şekilde ölçeklendirilmiş):

| Yıl | Toplam Arz (Tahmini) | Yıllık Enflasyon Oranı | Staking Ödülleri | Hazine Tahsisi | 
| :-- | :------------------- | :--------------------- | :--------------- | :------------- | 
| 1   | 100M HEZ             | %10.0                  | 8.5M HEZ         | 1.5M HEZ       | 
| 5   | 150M HEZ             | %6.7                   | 8.5M HEZ         | 1.5M HEZ       | 
| 10  | 200M HEZ             | %5.0                   | 8.5M HEZ         | 1.5M HEZ       | 
| 20  | 300M HEZ             | %3.3                   | 8.5M HEZ         | 1.5M HEZ       | 

Not: Gerçek emisyon oranları, yönetişim yoluyla belirlenecek ve ağ güvenlik ihtiyaçlarına göre ayarlanacaktır.

### 5.3.2. PEZ Dağıtım Zaman Çizelgesi

| Döngü | Yıllar | Emisyon Oranı | Kümülatif Dağıtım           | Kalan Arz          | 
| :---- | :----- | :------------ | :-------------------------- | :----------------- | 
| Genesis | 0      | -             | 1.2B (Hazine+Ön Satış+Kurucu) | 3.8B               | 
| 1     | 0-4    | %100          | +1.9B                       | 1.9B               | 
| 2     | 4-8    | %50           | +950M                       | 950M               | 
| 3     | 8-12   | %25           | +475M                       | 475M               | 
| 4     | 12-16  | %12.5         | +237.5M                     | 237.5M             | 
| 5     | 16-20  | %6.25         | +118.75M                    | 118.75M            | 

### 5.3.3. Staking Katılım Senaryoları

| Senaryo          | Staking Oranı | Yıllık HEZ Ödülü (1000 HEZ başına) | Ağ Güvenliği       | Likidite Durumu | 
| :--------------- | :------------ | :--------------------------------- | :-----------       | :-------------- | 
| Düşük Katılım    | %30           | ~28 HEZ                            | ⚠ Savunmasız      | ✅ Yüksek       | 
| Optimal          | %60           | ~14 HEZ                            | ✅ Güvenli        | ✅ Dengeli      | 
| Yüksek Katılım   | %80           | ~10.6 HEZ                          | ✅ Çok Güvenli    | ⚠ Düşük         | 

---

## 6. Core Features & Custom Pallets

PezkuwiChain, Polkadot SDK'nın temel işlevselliğini, her biri Kürt dijital devlet ekosistemi içindeki belirli bir ihtiyacı karşılamak üzere tasarlanmış bir dizi özel yapım pallet ile genişletir.

| Pallet                     | Açıklama                                                                                                     | Temel Özellikler                               | 
| :------------------------- | :----------------------------------------------------------------------------------------------------------- | :--------------------------------------------- | 
| `identity-kyc`             | Güvenli ve doğrulanabilir vatandaş kaydı ve KYC/AML süreçlerini sağlayan kapsamlı bir dijital kimlik sistemi. | Egemen Kimlik, KYC/AML                       | 
| `welati`                   | PezkuwiChain'in demokratik yönetişiminin temel taşı, zincir içi teklifler, oylama ve referandumları sağlar. Adı Kürtçe'de "vatandaş" anlamına gelir. | Zincir içi oylama, Parlamento, Bakanlar Kurulu | 
| `perwerde`                 | Sertifikaların verilmesi ve doğrulanması için güvene dayalı bir yol sağlayan merkeziyetsiz bir eğitim ve sertifikasyon platformu. Adı Kürtçe'de "eğitim" anlamına gelir. | Sertifika verme, Eğitim takibi                | 
| `pez-treasury`             | Hazine (20.25%), ön satış (1.875%) ve kurucu (1.875%) için hardcoded tahsisatlarla PEZ token hazinesini yönetir. 48 aylık sentetik yarılanma mekanizmasını uygular. | Hazine yönetimi, PEZ tahsisi                  | 
| `pez-rewards`              | Güven puanlarına dayalı ödül dağıtım mekanizmasını uygular. Dönem tabanlı ödüller (432.000 blok/dönem), Parlamenter NFT ödülleri (%10 teşvik havuzu için 201 NFT sahibi), ve talep dönemleri (100.800 blok) içerir. Güven ağırlıklı ödüller için `pallet-trust` ile entegredir. | Güven ağırlıklı ödüller, Talep dönemleri        | 
| `validator-pool`           | Kullanıcıların ağ güvenliğine katılım sürecini basitleştiren bir validatör havuzu yönetim sistemi.      | Havuz yönetimi, Koordinasyon                 | 
| `staking-score`            | Validatörler ve adaylar için bir performans ve itibar sistemi, ağ sağlığını ve güvenilirliğini teşvik eder. | Performans takibi, İtibar puanlaması          | 
| `trust`                    | TNPoS'u güçlendiren sosyal güven ve itibar sistemi. Kullanıcıların güven ağları oluşturmasına ve validatör seçimi ile ödül dağıtımında kullanılan güven puanları sağlamasına olanak tanır. | Güven puanlaması, İtibar entegrasyonu          | 
| `referral`                 | Referans ödülleri aracılığıyla kullanıcı büyümesini ve ağ benimsemeyi teşvik eden bir mekanizma.         | Referans takibi, Teşvikler                   | 
| `tiki`                     | Benzersiz ekosistem özellikleri için özel bir pallet.                                                     | Özel işlevsellik                             | 
| `pallet-presale` (Yeni)    | Çoklu eş zamanlı ön satışlar, platform ücreti, iade sistemi, vesting ve bonus katmanları sunan kapsamlı bir ön satış başlatma platformu. | Çoklu ön satış, Vesting, İade sistemi          | 
| `pallet-token-wrapper` (Yeni)| Yerel tokenleri (HEZ) sarılmış varlıklara (wHEZ) dönüştürmek için bir pallet, yerel ve varlık tokenleri arasında DEX işlemlerini etkinleştirir. | HEZ -> wHEZ dönüştürme                       | 

### 6.1. Parlamenter NFT Sistemi
PezkuwiChain, `pallet-pez-rewards` ile entegre edilmiş yeni bir Parlamenter NFT sistemi sunar. Bu sistem şunlardan oluşur:

**Teknik Sabitler (`pallet-pez-rewards`'den):**
```rust
pub const PARLIAMENTARY_COLLECTION_ID: u32 = 100;
pub const PARLIAMENTARY_NFT_COUNT: u32 = 201;
pub const PARLIAMENTARY_REWARD_PERCENT: u32 = 10; // %10 teşvik havuzu
```
**Özellikler:**
*   **Koleksiyon Kimliği:** 100
*   **NFT Sayısı:** 201 benzersiz Parlamenter NFT.
*   **Ödül Tahsisi:** Teşvik havuzunun %10'u Parlamenter NFT sahipleri arasında dağıtılır.
*   **Yönetişim Rolü:** NFT sahipleri parlamento koltuklarını veya özel yönetişim rollerini temsil edebilir.

Bu mekanizma, sınırlı NFT sahiplerinin ek ödüller ve potansiyel olarak `welati` yönetişim sisteminde geliştirilmiş oy gücü aldığı benzersiz bir yönetişim ve teşvik yapısı oluşturur.

### 6.2. Dönem ve Ödül Mekanikleri

**Teknik Sabitler (`pallet-pez-rewards`'den):**
```rust
pub const BLOCKS_PER_EPOCH: u32 = 432_000; // ~30 days
pub const CLAIM_PERIOD_BLOCKS: u32 = 100_800; // ~7 days
```
**Ödül Dağıtım Akışı:**
1.  **Dönem Süresi:** 432.000 blok (~30 gün, 6 saniyelik blok süresinde).
2.  **Ödül Hesaplama:** `pallet-trust`'tan alınan güven puanlarına göre.
3.  **Talep Dönemi:** Ödülleri talep etmek için 100.800 blok (~7 gün).
4.  **Geri Alma (Clawback):** Talep edilmeyen ödüller, talep süresi sona erdikten sonra belirlenen alıcıya iade edilir.
5.  **Parlamenter Bonus:** Havuzun %10'u 201 NFT sahibine dağıtılır.

---

## 7. Technical Specifications

PezkuwiChain'in teknik mimarisi, Polkadot ekosisteminin en son teknolojilerinden yararlanarak güvenlik, ölçeklenebilirlik ve birlikte çalışabilirlik için tasarlanmıştır.

| Spesifikasyon             | Değer                               | Açıklama                                      | 
| :------------------------ | :---------------------------------- | :---------------------------------------------| 
| Çerçeve                   | Polkadot SDK / Substrate            | Modüler blockchain çerçevesi                  | 
| Programlama Dili          | Rust                                | Bellek güvenli sistem programlama             | 
| Konsensüs                 | TNPoS (BABE + GRANDPA + Trust)      | Güven geliştirilmiş Nominated Proof-of-Stake  | 
| Blok Süresi               | ~6 saniye                           | BABE slot tabanlı üretim                      | 
| Kesinlik                  | ~12-18 saniye                       | GRANDPA kesinlik aracı                        | 
| Dönem Süresi (Epoch)      | 432.000 blok (~30 gün)              | PEZ ödül dağıtım dönemi                       | 
| Era Süresi                | ~24 saat                            | HEZ stake ödül dönemi                         | 
| Unbonding Süresi          | ~28 gün                             | Slashing için güvenlik tamponu                | 
| Runtime Ortamı            | WebAssembly (Wasm)                  | Çatallanmasız yükseltmeler                    | 
| Ağ İletişimi              | Libp2p                              | Modüler P2P ağ iletişimi                      | 
| Veritabanı                | RocksDB                             | Kalıcı durum depolaması                       | 
| Parachain Uyumluluğu      | Cumulus (Relay, Asset Hub, People)  | Polkadot/Kusama entegrasyonu                  | 
| Zincirler Arası İletişim  | XCM                                 | Zincirler Arası Konsensüs Mesajlaşması        | 
| HEZ Ondalık Basamakları   | 10                                  | 1 HEZ = 10^10 Planck                          | 
| PEZ Ondalık Basamakları   | 12                                  | 1 PEZ = 10^12 birim                           | 
| Maksimum Validatörler     | Yapılandırılabilir (100 ile başlar) | Ağ büyümesiyle ölçeklenir                     | 
| Maksimum Adaylar          | Sınırsız                            | Adaylık havuzları aracılığıyla                | 

---

## 8. Network Architecture

PezkuwiChain ağı, blockchain'in bütünlüğünü ve güvenliğini sürdürmek için birlikte çalışan merkeziyetsiz bir node sistemidir. Mimari, dayanıklı, ölçeklenebilir ve sansür dirençli olacak şekilde tasarlanmıştır.

### 8.1. Node Tipleri
**Validatör Node'ları:** Blok üreterek (BABE aracılığıyla) ve kesinlik için oylama yaparak (GRANDPA aracılığıyla) konsensüse katılan tam node'lardır. Validatörler HEZ tokenlerini stake etmeli ve yüksek çalışma süresi ve performans sürdürmelidir. TNPoS sistemindeki stake ve güven puanlarına göre seçilirler.

**Aday Node'ları:** Güvenilen validatörleri desteklemek için HEZ tokenlerini stake eden katılımcılar. Adaylar, seçtikleri validatörler tarafından kazanılan ödülleri paylaşırlar ve validatörleri kötü davrandığında slashing'e tabidirler.

**Tam Node'lar:** Blockchain durumunun tam bir kopyasını tutan, doğrulamayan node'lardır. RPC isteklerini karşılar, işlem aktarır ve blokları ağ genelinde yaymaya yardımcı olurlar.

**Hafif İstemciler:** Yalnızca blok başlıklarını indiren ve talep üzerine belirli verileri isteyen hafif node'lardır. Tam durumu depolamadan zincirle etkileşim kurmak için mobil ve tarayıcı tabanlı uygulamaları etkinleştirirler.

### 8.2. Ağ Topolojisi
Ağ, eşler arası iletişim için Libp2p'yi kullanır ve şunları sağlar:
*   **Eş Keşfi:** Yerel ağlar için DHT tabanlı keşif ve mDNS.
*   **Taşıma Güvenliği:** Şifreli bağlantılar için Gürültü protokolü.
*   **Çoklama:** Verimli bağlantı yönetimi için Yamux.
*   **NAT Geçişi:** Güvenlik duvarı atlaması için otomatik delik açma.

### 8.3. Telemetri ve İzleme
PezkuwiChain node'ları isteğe bağlı olarak halka açık gösterge panolarına telemetri verileri bildirebilir, bu da ağ sağlığı, validatör performansı ve coğrafi dağıtım konusunda şeffaflık sağlar.

---

## 9. Governance Model

PezkuwiChain, topluluğunu güçlendiren tamamen merkeziyetsiz, zincir içi bir yönetişim modeline kendini adamıştır. `welati` paleti, teklifler, oylama ve ağ yükseltmelerinin otonom olarak yürürlüğe konması için çerçeve sağlayan bu demokratik sistemin temel taşıdır.

### 9.1. Yönetişim Mekanizmaları
**Teklifler:** Her PEZ sahibi, ağ değişiklikleri, hazine harcamaları veya politika kararları için bir teklif sunabilir. Teklifler, spam'i önlemek için minimum bir depozito gerektirir.

**Konsey:** Hızlı teklifleri takip edebilen ve pasif paydaşları temsil edebilen seçilmiş bir temsilci organıdır. Konsey üyeleri onay oylamasıyla seçilir.

**Teknik Komite:** Acil hata düzeltmelerini ve güvenlik yamalarını hızlı bir şekilde takip edebilen bir çekirdek geliştiriciler ve teknik uzmanlar grubudur.

**Referandumlar:** Tüm büyük kararlar, tüm PEZ sahiplerinin katılabileceği halk oylamasına sunulur. Oy gücü stake ile ağırlıklandırılır ve ikna (daha güçlü oylar için tokenleri zaman kilitleme) ile artırılabilir.

**Yürürlüğe Koyma:** Onaylanan teklifler, kritik sorunlar tespit edilirse acil durum iptaline izin veren bir gecikme süresinden sonra zincir üzerinde otomatik olarak yürürlüğe konur.

### 9.2. Hazine Yönetimi
`pez-treasury` paleti, 1.012.500.000 PEZ (%20.25 toplam arz) hardcoded tahsisatı ile merkeziyetsiz bir hazineyi yönetir. Ek olarak, HEZ enflasyonunun %15'i hazineye akar. Bu hazine, genesis'te ve devam eden enflasyon yoluyla finanse edilir ve ağı faydalı projeleri ve girişimleri finanse etmek için tahsis edilebilir.

**Harcama Teklifleri:** Topluluk üyeleri, geliştirme, pazarlama, araştırma veya diğer ekosistem faydaları için hazine fonu talep etmek üzere teklifler sunabilir.

**Onay Süreci:** Harcama teklifleri, PEZ sahiplerinin ve/veya Konsey'in onayını gerektiren yönetişim mekanizması aracılığıyla oylanır.

**Yakma Mekanizması:** Harcanmayan hazine fonları, yönetişim kararlarına tabi olarak deflasyonist baskı oluşturmak için periyodik olarak yakılabilir.

### 9.3. Güven Geliştirilmiş Yönetişim
`pallet-trust` sisteminin yönetişime entegrasyonu benzersiz dinamikler yaratır:
*   **Güven Ağırlıklı Oylama:** Daha yüksek güven puanına sahip katılımcılar, artırılmış oy gücü alabilir.
*   **İtibar Tabanlı Teklifler:** Yüksek güvene sahip üyeler, daha düşük teklif depozitolarına sahip olabilir.
*   **Validatör Yönetişimi:** Kanıtlanmış geçmişe sahip validatörler, teknik kararlarda daha güçlü etkiye sahiptir.

### 9.4. Risk Faktörleri ve Azaltma Stratejileri
PezkuwiChain güvenlik ve esneklik göz önünde bulundurularak tasarlanmış olsa da, tüm blockchain sistemleri doğal risklerle karşı karşıyadır. Bu bölüm, birincil risk kategorilerini ve uygulanan azaltma stratejilerini özetlemektedir.

### 9.4.1. Teknik Riskler
**Akıllı Sözleşme Zafiyetleri:**
*   **Risk:** Runtime pallet'lerindeki hatalar, açıklara, fon kaybına veya ağ kesintisine yol açabilir.
*   **Azaltma:** 
    *   Kapsamlı birim ve entegrasyon testleri.
    *   Kritik pallet'lerin resmi doğrulaması.
    *   Saygın firmalar tarafından bağımsız güvenlik denetimleri.
    *   Önemli ödüllerle hata ödül programı.
    *   Testnet doğrulaması ile kademeli dağıtım.

**Runtime Yükseltme Başarısızlıkları:**
*   **Risk:** Çatallanmasız yükseltmeler, uyumsuzluklara veya kritik değişikliklere yol açabilir.
*   **Azaltma:** 
    *   Mainnet dağıtımından önce testnetlerde kapsamlı testler.
    *   Geri alma özelliklerine sahip çok aşamalı yükseltme süreci.
    *   Acil durum düzeltmeleri için Teknik Komite denetimi.
    *   Yürürlüğe koymadan önce topluluk inceleme süresi.

**Konsensüs Başarısızlıkları:**
*   **Risk:** TNPoS uygulama hataları, kesinliği veya canlılığı tehlikeye atabilir.
*   **Azaltma:** 
    *   Sahada test edilmiş Polkadot SDK temeline dayalı inşa etme.
    *   Artımlı güven sistemi entegrasyonu.
    *   Güven sistemi başarısız olursa standart NPoS'a geri dönüş.
    *   Sürekli izleme ve uyarı.

### 9.4.2. Ekonomik Riskler
**Token Volatilitesi:**
*   **Risk:** HEZ ve PEZ fiyat dalgalanmaları ağ güvenliğini ve benimsenmesini etkileyebilir.
*   **Azaltma:** 
    *   Çift token modeli, faydayı yönetişimden ayırır.
    *   Staking teşvikleri tutarlı bir güvenlik bütçesi sağlar.
    *   Piyasa stresinde stabilize etmek için hazine rezervleri.
    *   Yarılanma mekanizması aracılığıyla kademeli token dağıtımı.

**Yetersiz Staking Katılımı:**
*   **Risk:** Düşük staking oranları ağ güvenliğini tehlikeye atabilir.
*   **Azaltma:** 
    *   Dinamik enflasyon, staking'i teşvik etmek için ödülleri ayarlar.
    *   Aday havuzları giriş engelini düşürür.
    *   Güvene dayalı ödüller ek teşvikler yaratır.
    *   Staking faydaları üzerine eğitim kampanyaları.

**Hazine Tüketimi:**
*   **Risk:** Aşırı harcama hazine rezervlerini tüketebilir.
*   **Azaltma:** 
    *   Tüm harcamalar için yönetişim onayı gereklidir.
    *   Çift fonlama (PEZ + HEZ enflasyonu).
    *   Harcanmayan fonların periyodik olarak yakılması.
    *   Uzun vadeli bütçe planlaması.

### 9.4.3. Yönetişim Riskleri
**Düşük Seçmen Katılımı:**
*   **Risk:** İlgisizlik, küçük azınlıkların kararlar almasına yol açabilir.
*   **Azaltma:** 
    *   Güven ağırlıklı oylama, aktif katılımcıları ödüllendirir.
    *   Pasif paydaşlar için Konsey temsilciliği.
    *   Kullanıcı dostu yönetişim arayüzleri.
    *   Topluluk katılımı ve eğitim.

**Kötü Niyetli Teklifler:**
*   **Risk:** Saldırganlar zararlı teklifler sunabilir.
*   **Azaltma:** 
    *   Teklif depozitoları ekonomik engeller yaratır.
    *   Topluluk incelemesi ve tartışma dönemleri.
    *   Konsey ve Teknik Komite veto yetkileri.
    *   Acil durum iptal mekanizmaları.

**Yönetişim Ele Geçirme:**
*   **Risk:** Zengin aktörler karar alma sürecine hakim olabilir.
*   **Azaltma:** 
    *   Güven puanları saf plütokrasiyi dengeler.
    *   Mahkumiyet oylaması uzun vadeli taahhüdü ödüllendirir.
    *   Konseyde çeşitli paydaş temsiliyeti.
    *   Şeffaf zincir içi oylama kayıtları.

### 9.4.4. Operasyonel Riskler
**Validatör Merkezileşmesi:**
*   **Risk:** Coğrafi veya varlık yoğunlaşması ademi merkeziyetçiliği tehdit edebilir.
*   **Azaltma:** 
    *   Eşit temel ödüller stake yoğunlaşmasını önler.
    *   Güven puanları dağıtılmış, güvenilir validatörleri destekler.
    *   Aday havuzları geniş katılımı sağlar.
    *   Coğrafi çeşitlilik teşvikleri.

**Ağ Saldırıları:**
*   **Risk:** DDoS, eclipse veya uzun menzilli saldırılar.
*   **Azaltma:** 
    *   Libp2p'nin sağlam ağ katmanı.
    *   28 günlük unbonding süresi uzun menzilli saldırıları önler.
    *   Slashing kötü niyetli davranışları caydırır.
    *   Çeşitli validatör seti saldırı maliyetini artırır.

### 9.4.5. Legal & Compliance Riskleri
**Yasal Belirsizlik:**
*   **Risk:** Gelişen düzenlemeler operasyonları etkileyebilir.
*   **Azaltma:** 
    *   KYC/AML özellikli `identity-kyc` paleti.
    *   Şeffaf yönetişim ve hazine.
    *   Hukuk danışmanlığı ve uyumluluk izlemesi.
    *   Düzenleyici adaptasyon için esnek mimari.

### 9.5. Risk Yönetimi Çerçevesi
PezkuwiChain, sürekli bir risk yönetimi süreci uygular:
1.  **Tanımlama:** Düzenli güvenlik denetimleri ve topluluk raporlaması.
2.  **Değerlendirme:** Teknik Komite, şiddeti ve olasılığı değerlendirir.
3.  **Azaltma:** Teknik ve yönetişim çözümlerini uygular.
4.  **İzleme:** Ağ metriklerinin sürekli gözetimi.
5.  **Yanıt:** Kritik olaylar için acil durum prosedürleri.

---

## 10. Security and Auditing

Güvenlik, PezkuwiChain için her şeyden önemli bir konudur. Proje, Rust programlama dilinin seçiminden çekirdek protokollerinin tasarımına kadar çoklu güvenlik katmanlarından yararlanmaktadır.

### 10.1. Güvenlik Katmanları
**Dil Seviyesi Güvenliği:** Rust'ın bellek güvenliği garantileri, C/C++ blockchain uygulamalarını etkileyen tüm güvenlik açığı sınıflarını (buffer taşmaları, use-after-free, veri yarışları) ortadan kaldırır.

**Çerçeve Güvenliği:** Polkadot SDK, Polkadot, Kusama ve çok sayıda parachain genelinde milyarlarca doları güvence altına alarak sahada test edilmiştir.

**Çatallanmasız Yükseltmeler:** WebAssembly runtime'ı, ağ kesintisi veya çekişmeli sert çatallar olmadan güvenlik yamaları için olanak tanır.

**Ekonomik Güvenlik:** Slashing özellikli TNPoS, validatörlerin dürüst davranmaları için güçlü ekonomik teşviklere sahip olmalarını sağlar.

**Sosyal Güvenlik:** Güven sistemi, itibar katmanı ekleyerek saldırıları sadece ekonomik olarak değil, aynı zamanda sosyal olarak da maliyetli hale getirir.

### 10.2. Denetim Stratejisi
**Aşama 1 - Dahili Denetimler:** Çekirdek ekip, kapsamlı kod incelemeleri ve güvenlik analizleri yapar.

**Aşama 2 - Harici Denetimler:** Bağımsız güvenlik firmaları, kritik pallet'leri (`pez-treasury`, `pez-rewards`, `trust`, `identity-kyc`, `pallet-presale`, `pallet-token-wrapper`, `pallet-welati`, `pallet-perwerde`, `pallet-referral`, `pallet-tiki`, `pallet-staking-score`, `pallet-validator-pool`) denetler.

**Aşama 3 - Sürekli İzleme:** Sürekli güvenlik izleme ve olay müdahale prosedürleri.

### 10.3. Hata Ödül Programı
PezkuwiChain, şiddetine göre ölçeklendirilmiş ödüllerle bir hata ödül programı oluşturacaktır:

| Ciddiyet | Açıklama                                       | Ödül Aralığı          | 
| :------- | :--------------------------------              | :------------------   | 
| Kritik   | Konsensüs hatası, fon hırsızlığı, ağ durması   | 50.000 - 200.000 USD  | 
| Yüksek   | DoS saldırıları, ayrıcalık yükseltme           | 10.000 - 50.000 USD   | 
| Orta     | Bilgi ifşası, küçük açıklıklar                 | 2.000 - 10.000 USD    | 
| Düşük    | En iyi uygulama ihlalleri, kod kalitesi        | 500 - 2.000 USD       | 

### 10.4. Olay Müdahalesi
Bir güvenlik olayı durumunda:
1.  **Tespit:** Otomatik izleme ve topluluk raporlaması.
2.  **Değerlendirme:** Teknik Komite şiddeti ve olasılığı değerlendirir.
3.  **Sınırlama:** Gerekirse acil runtime yükseltmesi.
4.  **İyileştirme:** Yama dağıtımı ve validatör koordinasyonu.
5.  **Olay Sonrası:** Kamu açıklaması ve öğrenilen dersler.

### 10.5. Çevresel Sürdürülebilirlik
PezkuwiChain'in Proof-of-Stake konsensüsü, Proof-of-Work sistemlerine göre önemli çevresel avantajlar sağlar:

**Enerji Verimliliği Karşılaştırması:**

| Blockchain       | Konsensüs | Yıllık Enerji Tüketimi | kWh Başına İşlem Sayısı | 
| :--------------- | :-------- | :--------------------- | :---------------------- | 
| Bitcoin          | PoW       | ~150 TWh               | ~5                      | 
| Ethereum (önce)  | PoW       | ~100 TWh               | ~15                     | 
| Ethereum (sonra) | PoS       | ~0.01 TWh              | ~1.000.000              | 
| Polkadot         | NPoS      | ~0.005 TWh             | ~2.000.000              | 
| PezkuwiChain     | TNPoS     | ~0.003 TWh (tahmini)   | ~2.500.000 (tahmini)    | 

**Karbon Ayak İzi:** PezkuwiChain'in tahmini karbon ayak izi, Bitcoin'den yaklaşık %99.99 daha düşüktür, bu da onu en çevre dostu blockchain ağlarından biri yapar.

**Yeşil Introductionimler:**
*   Yenilenebilir enerji kullanımı için validatör teşvikleri.
*   Hazine tarafından finanse edilen karbon dengeleme programları.
*   Yeşil blockchain standartları için sertifikasyon çalışmaları.
*   Sürdürülebilir blockchain teknolojisi üzerine eğitim kampanyaları.

---

## 11. Roadmap & Development Phases (Updated)

PezkuwiChain, istikrarlı, güvenli ve özellik açısından zengin bir mainnet lansmanını sağlamak için aşamalı bir geliştirme yol haritası takip eder.

| Aşama                     | Durum             | Zaman Çizelgesi (Güncel)   | Temel Başarılar (Güncel)                                               | 
| :-------------------      | :-----------      | :-----------------------   | :--------------------------------------------------------------------  | 
| Alfa Testnet              | ✅ Tamamlandı     | Q4 2025                   | Çekirdek işlevselliklerin ilk ağ lansmanı ve başarılı dağıtımı.        | 
| Beta Testnet              | 🔄 Devam Ediyor   | Q1 2026                   | Beta testnet'in tanıtımı, stabilite ve validatör katılımına odaklanma. (`pallet-pez-treasury`, `pez-rewards` entegrasyonu). | 
| Staging Testnet           | 🔜 Yaklaşan       | Q2 2026                   | Kapsamlı Beta Testnet çıktıları, kullanıcı ve validatör kılavuzları.  | 
| Mainnet Lansmanı          | 🔜 Yaklaşan       | Q3 2026                   | PezkuwiChain egemen blockchain ağının resmi lansmanı.                 | 
| Parachain Entegrasyonu    | 🔮 Gelecek        | 2027                      | Polkadot veya Kusama relay chain'e parachain olarak bağlantı.         | 
| XCM Köprüleri             | 🔮 Gelecek        | 2027                      | Büyük blockchain ekosistemlerine zincirler arası köprüler.            | 

### 11.1. Mainnet Sonrası Yol Haritası
**Yıl 1 (2027):**
*   Ekosistem hibe programı başlatılması.
*   DApp geliştirici katılımı.
*   Hükümet hizmetleri tarafından `identity-kyc` benimsenmesi.
*   `perwerde` eğitim platformu pilot programları.

**Yıl 2 (2028):**
*   Parachain slot alımı (Polkadot veya Kusama).
*   Büyük parachain'lerle XCM entegrasyonu.
*   DeFi ekosistemi genişlemesi (DEX, borç verme, stablecoinler).
*   Mobil cüzdan ve hafif istemci sürümleri.

**Yıl 3 (2029+):**
*   Katman-2 ölçeklendirme çözümleri.
*   Gizlilik özellikleri (sıfır bilgi kanıtları).
*   Polkadot dışı zincirlerle birlikte çalışabilirlik.
*   Kürt diasporasının ötesinde küresel genişleme.

---

## 12. Use Cases & Applications

PezkuwiChain, Kürt ulusuna ve ötesine hizmet eden geniş bir yelpazedeki merkeziyetsiz uygulamalar ve hizmetler için temel bir katman olarak tasarlanmıştır.

### 12.1. Dijital Kimlik ve Vatandaşlık
**`identity-kyc` Paleti Uygulamaları:**
*   **Ulusal Dijital Kimlik:** Fiziksel belgelerin yerini alan, tüm vatandaşlar için egemen dijital kimlik.
*   **Sınırlar Arası Tanıma:** Kürt bölgelerinde kabul edilen doğrulanabilir kimlik bilgileri.
*   **KYC/AML Uyumluluğu:** Finansal kurumlar, kişisel verileri depolamadan kimliği doğrulayabilir.
*   **Sağlık Kayıtları:** Bireyler tarafından kontrol edilen güvenli, taşınabilir tıbbi kayıtlar.
*   **Oy Hakları:** `welati` yönetişim katılımı için kriptografik vatandaşlık kanıtı.

**Örnek Kullanım Durumu:** Diasporadaki bir Kürt vatandaşı, fiziksel belgeler olmadan devlet hizmetlerine erişmek, referandumlarda oy kullanmak ve banka hesabı açmak için kimliğini kanıtlayabilir.

### 12.2. Demokratik Yönetişim
**`welati` Paleti Uygulamaları:**
*   **Ulusal Referandumlar:** Anayasal ve politika konularında doğrudan demokrasi.
*   **Yerel Yönetişim:** Bölgesel ve belediye karar alma.
*   **Bütçe Tahsisi:** Kamu fonları için katılımcı bütçeleme.
*   **Şeffaflık:** Tüm teklifler, oylar ve harcamalar herkese açık olarak denetlenebilir.
*   **Delegasyon:** Güvenilen temsilcilere oy delegasyonuna izin veren likit demokrasi.

**Örnek Kullanım Durumu:** Yeni bir okulun finansmanı için bir topluluk teklifi sunulur, tartışılır, oylanır ve hazineden otomatik olarak finanse edilir - hepsi zincir üzerinde şeffaf bir şekilde.

### 12.3. Eğitim ve Kimlik Bilgileri
**`perwerde` Paleti Uygulamaları:**
*   **Akademik Sertifikalar:** Kurcalamaya karşı korumalı diplomalar ve dereceler.
*   **Profesyonel Lisanslar:** Doktorlar, mühendisler, avukatlar için doğrulanabilir kimlik bilgileri.
*   **Beceri Rozetleri:** Belirli yetkinlikler için mikro kimlik bilgileri.
*   **Sürekli Eğitim:** Yaşam boyu öğrenme kayıtları.
*   **İşveren Doğrulaması:** Kurumuyla iletişime geçmeden anında, güvene dayalı kimlik bilgisi doğrulaması, kimlik bilgisi sahtekarlığını ortadan kaldırır.

**Örnek Kullanım Durumu:** Bir üniversite, işverenlerin kurumla iletişime geçmeden anında doğrulayabileceği, kimlik bilgisi sahtekarlığını ortadan kaldıran blockchain tabanlı bir diploma verir.

### 12.4. Merkeziyetsiz Finans (DeFi)
**Finansal Altyapı:**
*   **Merkeziyetsiz Borsalar (DEX):** HEZ/PEZ ve diğer token ticareti.
*   **Borç Verme Protokolleri:** Teminat olarak HEZ kullanan teminatlı krediler.
*   **Stablecoinler:** Fiyat istikrarı için Kürt Dinarı'na sabitlenmiş stablecoinler.
*   **Sınırlar Arası Ödemeler:** Diaspora için anında, düşük maliyetli para transferleri.
*   **Getiri Tarımı (Yield Farming):** DeFi katılımcıları için likidite sağlama ödülleri.

**Örnek Kullanım Durumu:** Avrupa'daki bir işçi, Kurdistan'daki ailesine HEZ transferi aracılığıyla anında para göndererek, geleneksel bankacılığın yüksek ücretlerinden ve gecikmelerinden kaçınır.

### 12.5. Tedarik Zinciri ve Ticaret
*   **Menşe Doğrulaması:** Kürt ürünlerini kaynaktan tüketiciye kadar takip etme.
*   **Ticaret Finansmanı:** Uluslararası ticaret anlaşmaları için akıllı sözleşmeler.
*   **Gümrük ve Tarifeler:** Otomatik gümrük vergisi tahsilatı ve uyumluluk.
*   **Kalite Güvencesi:** Denetim ve sertifikasyonların değişmez kayıtları.

### 12.6. Sosyal Etki
*   **Hayırseverlik:** STK'lara şeffaf bağış takibi.
*   **Mülteci Yardımı:** Yerinden edilmiş kişiler için dijital kimlik.
*   **Arazi Kaydı:** Anlaşmazlıkları önleyen değişmez mülkiyet kayıtları.
*   **Kültürel Koruma:** Kürt sanatı, müziği ve edebiyatı için NFT'ler.

---

## 13. Team & Contributors

PezkuwiChain, ulusal kalkınma için en son teknolojiyi kullanma konusundaki güçlü taahhüdünü yansıtan Kürdistan Teknoloji Bakanlığı liderliğinde bir girişimdir. Proje, PezkuwiChain ekosistemini inşa etmeye uzmanlıklarını adamış küresel bir 156 katılımcı topluluğundan (Ekim 2025 itibarıyla) yararlanmaktadır.

### 13.1. Çekirdek Ekip
**Kürdistan Teknoloji Bakanlığı:** Projenin stratejik yönünü ve finansmanını denetleyen hükümet organı. Ulusal dijital altyapı hedefleriyle uyumu sağlar.

**Teknik Liderlik:** Polkadot, Substrate ve dağıtık sistemler alanında deneyimli blockchain mimarları ve Rust geliştiricileri.

**Araştırma Ekibi:** TNPoS konsensüsü ve tokenomik araştırmalarını ilerleten kriptograflar, ekonomistler ve bilgisayar bilimcileri.

**Topluluk Yöneticileri:** Küresel Kürt geliştirici ve kullanıcı topluluğunu inşa etmek ve beslemek.

### 13.2. Katılımcılar
Projenin 156 katılımcısı birden fazla disiplini kapsar:
*   **Çekirdek Geliştiriciler:** Runtime geliştirme, pallet uygulaması, konsensüs mühendisliği.
*   **Altyapı:** Node operatörleri, DevOps, ağ izleme.
*   **Dokümantasyon:** Teknik yazarlar, çevirmenler (Kürtçe, İngilizce, Arapça, Türkçe).
*   **Tasarım:** Cüzdanlar ve dApp'ler için UI/UX tasarımcıları.
*   **Topluluk:** Moderatörler, eğitimciler, etkinlik organizatörleri.

### 13.3. Danışmanlar
**Teknik Danışmanlar:** Polkadot ekosistem veteranları, Substrate uzmanları ve güvenlik araştırmacıları.

**Ekonomik Danışmanlar:** Tokenomik uzmanları ve finansal ekonomistler.

**Hukuk Danışmanları:** Blockchain düzenlemesi ve uyumluluk uzmanları.

**Kültürel Danışmanlar:** Kürt topluluğu liderleri, kültürel uyumu sağlar.

---

## 14. Ecosystem & Partnerships

PezkuwiChain, canlı ve birbirine bağlı bir ekosistem olarak tasarlanmıştır. Polkadot SDK üzerindeki temeli, dünyanın en aktif blockchain ekosistemlerinden biriyle doğal birlikte çalışabilirlik sağlar.

### 14.1. Polkadot Ekosistem Entegrasyonu
**Parachain Hazır Mimarisi:** PezkuwiChain, Cumulus ile inşa edilmiştir ve Polkadot veya Kusama relay zincirlerine bir parachain olarak bağlanmaya hazırdır. Bu, şunları sağlayacaktır:
*   **Paylaşımlı Güvenlik:** Relay zincirinin validatör setinin ekonomik güvenliğinden yararlanma.
*   **Birlikte Çalışabilirlik:** Yüzlerce diğer parachain ile doğal XCM iletişimi.
*   **Ölçeklenebilirlik:** Polkadot ağı genelinde paralel işlem işleme.

**XCM Entegrasyonu:** Zincirler Arası Konsensüs Mesajlaşması şunları sağlar:
*   PezkuwiChain ve diğer parachainler arasında varlık transferleri.
*   Uzaktan akıllı sözleşme çağrıları.
*   Zincirler arası yönetişim katılımı.
*   Paylaşımlı likidite havuzları.

### 14.2. Stratejik Ortaklıklar
**Eğitim Kurumları:**
*   `perwerde` kimlik bilgisi verme pilotları için üniversiteler.
*   Beceri sertifikasyon programları için mesleki okullar.
*   Blockchain teknolojisi üzerine araştırma işbirlikleri.

**Devlet Kurumları:**
*   `identity-kyc` benimsenmesi için bölgesel hükümetler.
*   `welati` yönetişim entegrasyonu için bakanlıklar.
*   Tedarik zinciri takibi için gümrük ve ticaret departmanları.

**STK'lar ve Sivil Toplum:**
*   Mülteci yardımı için insani yardım kuruluşları.
*   Mirasın korunması için kültürel kuruluşlar.
*   Topluluk katılımı için diaspora dernekleri.

**Özel Sektör:**
*   DeFi entegrasyonu için finans kurumları.
*   dApp geliştirme için teknoloji şirketleri.
*   Node barındırma için altyapı sağlayıcıları.

### 14.3. Geliştirici Ekosistemi
**Hibe Programı:** Hazine tarafından finanse edilen hibeler:
*   DApp geliştirme.
*   Altyapı araçları.
*   Eğitim içeriği.
*   Topluluk projeleri.

**Hackathonlar:** İnovasyonu teşvik etmek için düzenli kodlama yarışmaları.

**Kuluçka Merkezi:** PezkuwiChain üzerinde inşa eden startup'lar için destek.

**Dokümantasyon:** Kapsamlı kılavuzlar, eğitimler ve API referansları.

---

## 15. Legal & Compliance

PezkuwiChain, ademi merkeziyetçilik ve egemenlik ilkelerini korurken sorumlu ve uyumlu bir şekilde faaliyet göstermeyi taahhüt eder.

### 15.1. Lisanslama
Proje, yeniliği ve işbirliğini teşvik eden açık ve izinli bir lisans olan Kurdistan Yetenek Enstitüsü Lisansı altında faaliyet göstermektedir. Kod tabanı açık kaynak olup, topluluk denetimine ve katkılarına açıktır.

### 15.2. Düzenleyici Yaklaşım
**KYC/AML Uyumluluğu:** `identity-kyc` paleti, genel kullanıcılar için gizliliği korurken düzenleyici uyumluluk (örn: borsalar, finansal hizmetler) gerektiren uygulamalar için isteğe bağlı KYC/AML yetenekleri sağlar.

**Menkul Kıymet Uyumluluğu:** HEZ ve PEZ, ağ işlemleri ve yönetişim için fayda tokenları olarak tasarlanmıştır, yatırım menkul kıymetleri olarak değil. Ancak, proje düzenleyici çerçevelerin yargı alanına göre değiştiğini kabul eder ve operasyonların gerçekleştiği yerde uyumluluğa bağlıdır.

**Veri Koruması:** Mimari, kullanıcıların kişisel verilerini kontrol etmelerine ve GDPR gibi düzenlemelere uymalarına izin veren gizlilik odaklı olarak tasarlanmıştır.

### 15.3. Yasal Uyarı
**Yatırım Riski:** Bu whitepaper yalnızca bilgilendirme amaçlıdır ve bir satış teklifi, bir satın alma teklifi veya herhangi bir menkul kıymet veya başka bir ürün veya hizmet tavsiyesi teşkil etmez. PEZ ve HEZ tokenleri, PezkuwiChain ekosistemi içinde kullanılmak üzere tasarlanmış fayda tokenlarıdır. Yatırım araçları olması amaçlanmamıştır. Potansiyel katılımcılar, PezkuwiChain ağına katılmadan önce yasal ve finansal danışmanlarına danışmalıdır.

**Garanti Yok:** PezkuwiChain güvenlik ve güvenilirlik göz önünde bulundurularak tasarlanmış olsa da, hiçbir blockchain sistemi mükemmel güvenlik veya çalışma süresi garanti edemez. Kullanıcılar kendi riskleriyle katılırlar.

**Düzenleyici Belirsizlik:** Blockchain teknolojisi için düzenleyici ortam gelişmektedir. Kanun veya düzenlemelerdeki değişiklikler PezkuwiChain'in işleyişini veya faydasını etkileyebilir.

**İleriye Dönük Beyanlar:** Bu whitepaper, gelecekteki geliştirme, benimseme ve performansla ilgili ileriye dönük beyanlar içerir. Gerçek sonuçlar tahminlerden önemli ölçüde farklılık gösterebilir.

---

## 16. Conclusion

PezkuwiChain, Kürt ulusu için bir paradigma değişimi ve yenilikçi TNPoS konsensüs mekanizması aracılığıyla blockchain teknolojisine önemli bir katkı sağlamaktadır. Sağlam bir teknik mimariyi, düşünceli bir ekonomik ve sosyal vizyonu birleştirerek ve sosyal güveni çekirdek konsensüs katmanına entegre ederek, PezkuwiChain yeni bir dijital devlet için temel katmanı sağlar.

**Temel Başarılar:**
*   **Teknik İnovasyon:** TNPoS konsensüsü, sosyal itibarı ekonomik güvenlikle entegre eder.
*   **Ekonomik Sürdürülebilirlik:** Çift token modeli, faydayı ve yönetişimi dengeler.
*   **Kültürel Uyum:** Kimlik, yönetişim ve eğitim için özel palletler Kürt ihtiyaçlarına hizmet eder.
*   **Küresel Standartlar:** Birlikte çalışabilirlik ve güvenlik ile Polkadot SDK üzerine inşa edilmiştir.

**Gelecek Vizyonu:**
PezkuwiChain, Kürt ulusunun dijital omurgası olmayı hedefleyerek şunları sağlar:
*   **Finansal Katılım:** DeFi ve dijital para birimleri aracılığıyla bankasızları bankacılıkla buluşturma.
*   **Demokratik Katılım:** Şeffaf zincir içi yönetişim aracılığıyla doğrudan demokrasi.
*   **Eğitimsel İlerleme:** Doğrulanabilir kimlik bilgileri ve yaşam boyu öğrenme kayıtları.
*   **Ekonomik Fırsat:** Introductionimciler ve geliştiriciler için gelecek inşa etme platformu.

Bu, topluluk tarafından, topluluk için inşa edilen bir platformdur ve nihai hedefi, blockchain teknolojisinin dönüştürücü gücü aracılığıyla Kürdistan'ı güçlendirmektir. Ağ büyüdükçe ve olgunlaştıkça, PezkuwiChain, tek beden herkese uyar çözümleri dayatmak yerine, blockchain'in ulusların ve toplulukların özel ihtiyaçlarına nasıl hizmet edebileceğine dair bir model olarak hizmet edecektir.

Konseptten mainnet'e giden yolculuk, titiz geliştirme, topluluk işbirliği ve dijital egemenlik vizyonuna sarsılmaz bir bağlılıkla işaretlenmiştir. Yaklaşan mainnet lansmanı ile PezkuwiChain, vaatten gerçeğe geçerek Kürt halkına ve dünyaya merkeziyetsiz dijital altyapı için yeni bir paradigma sunacaktır.

---

## 17. References

### Akademik ve Teknik Makaleler
1.  Polkadot: Vision for a Heterogeneous Multi-Chain Framework - Dr. Gavin Wood, 2016. [https://polkadot.network/whitepaper/](https://polkadot.network/whitepaper/)
2.  BABE: Blind Assignment for Blockchain Extension - Web3 Foundation Research. [https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html](https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html)
3.  GRANDPA: A Byzantine Finality Gadget - Web3 Foundation Research. [https://research.web3.foundation/en/latest/polkadot/finality.html](https://research.web3.foundation/en/latest/polkadot/finality.html)
4.  Nominated Proof-of-Stake (NPoS) - Web3 Foundation. [https://wiki.polkadot.network/docs/learn-npos](https://wiki.polkadot.network/docs/learn-npos)
5.  XCM: The Cross-Consensus Message Format - Polkadot Wiki. [https://wiki.polkadot.network/docs/learn-xcm](https://wiki.polkadot.network/docs/learn-xcm)
6.  Substrate: A Blockchain Framework for a Multichain Future - Parity Technologies. [https://substrate.io/](https://substrate.io/)

### Proje Kaynakları
1.  PezkuwiChain GitHub Repository - [https://github.com/pezkuwichain/pezkuwi-sdk](https://github.com/pezkuwichain/pezkuwi-sdk)
2.  `pallet-pez-treasury` Source Code - [https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/pez-treasury](https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/pez-treasury)
3.  `pallet-pez-rewards` Source Code - [https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/pez-rewards](https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/pez-rewards)
4.  `pallet-trust` Source Code - [https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/trust](https://github.com/pezkuwichain/pezkuwi-sdk/tree/main/pezkuwi/pallets/trust)

### Blockchain Yönetişim ve Ekonomi
1.  On-Chain Governance - Vlad Zamfir, 2017. [https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)
2.  Tokenomics: The Economics of Cryptocurrencies - Shermin Voshmgir, 2020.
3.  Decentralized Autonomous Organizations (DAOs) - Vitalik Buterin, 2014.

### Güven ve İtibar Sistemleri
1.  The Eigentrust Algorithm for Reputation Management in P2P Networks - Kamvar et al., 2003.
2.  A Survey of Trust and Reputation Systems for Online Service Provision - Jøsang et al., 2007.
3.  Blockchain-Based Reputation Systems: A Survey - Dennis and Owenson, 2016.

### Dijital Kimlik
1.  Decentralized Identifiers (DIDs) v1.0 - W3C Recommendation, 2022. [https://www.w3.org/TR/did-core/](https://www.w3.org/TR/did-core/)
2.  Verifiable Credentials Data Model - W3C Recommendation, 2022. [https://www.w3.org/TR/vc-data-model/](https://www.w3.org/TR/vc-data-model/)

### Ek Dokümantasyon
1.  Polkadot SDK Documentation - [https://docs.polkadot.com/develop/parachains/intro-polkadot-sdk/](https://docs.polkadot.com/develop/parachains/intro-polkadot-sdk/)
2.  Rust Programming Language - [https://www.rust-lang.org/](https://www.rust-lang.org/)

---

## 18. Contact & Resources

### Resmi Kanallar
*   **Web Sitesi:** [https://pezkuwichain.io](https://pezkuwichain.io)
*   **GitHub:** [https://github.com/pezkuwichain/pezkuwi-sdk](https://github.com/pezkuwichain/pezkuwi-sdk)
*   **Dokümantasyon:** [https://docs.pezkuwichain.io](https://docs.pezkuwichain.io) -> **Güncel:** [https://docs.pezkuwichain.app](https://docs.pezkuwichain.app)
*   **Blok Gezgini:** [https://explorer.pezkuwichain.io](https://explorer.pezkuwichain.io)

### E-posta İletişimi
*   **Genel Sorular:** [info@pezkuwichain.io](mailto:info@pezkuwichain.io)
*   **Teknik Destek:** [tech@pezkuwichain.io](mailto:tech@pezkuwichain.io)
*   **Ortaklıklar:** [partnerships@pezkuwichain.io](mailto:partnerships@pezkuwichain.io)
*   **Hükümet İlişkileri:** [tech@kurdistan.gov](mailto:tech@kurdistan.gov)

### Sosyal Medya
*   **Twitter/X:** @PezkuwiChain -> **Güncel:** `[Buraya doğru Twitter/X linki gelecek]`
*   **Telegram:** [t.me/PezkuwiChain](https://t.me/PezkuwiChain)
*   **Discord:** [discord.gg/pezkuwichain](https://discord.gg/pezkuwichain)
*   **Medium:** medium.com/@pezkuwichain -> **Güncel:** `[Buraya doğru Medium linki gelecek]`

### Geliştirici Kaynakları
*   **Geliştirici Portalı:** [https://developers.pezkuwichain.io](https://developers.pezkuwichain.io)
*   **API Dokümantasyonu:** [https://api.pezkuwichain.io/docs](https://api.pezkuwichain.io/docs)
*   **Testnet Faucet:** [https://faucet.pezkuwichain.io](https://faucet.pezkuwichain.io)
*   **Hibe Programı:** [https://grants.pezkuwichain.io](https://grants.pezkuwichain.io)

---

## 19. Appendix A: Glossary

**BABE (Blind Assignment for Blockchain Extension):** Blok oluşturma için validatörlere blok oluşturma slotlarını rastgele atayan, sansür direncini ve tutarlı blok sürelerini sağlayan bir blok üretim mekanizmasıdır. PezkuwiChain yaklaşık 6 saniyeyi hedefler.

**Block Time:** Blockchain üzerinde ardışık bloklar arasındaki ortalama süre. PezkuwiChain ~6 saniyeyi hedefler.

**Clawback:** Talep edilmeyen ödüllerin, talep süresi sona erdikten sonra teşvik havuzuna geri döndürüldüğü mekanizmadır.

**Consensus:** Dağıtılmış bir ağın blockchain'in mevcut durumu üzerinde anlaştığı süreç.

**Cumulus:** Substrate zincirlerinin parachain olmasını sağlayan bir Polkadot SDK kütüphanesi.

**Era:** Stake etme sisteminde bir dönem (genellikle ~24 saat), bu süreden sonra HEZ için stake ödülleri hesaplanır ve dağıtılır.

**Epoch:** PEZ ödül dağıtımı için kullanılan daha uzun bir dönem (432.000 blok, ~30 gün).

**Era Points:** Validatörlerin zincir içi faaliyetlerine göre kazandığı performans metrikleri, bonus ödüllerini hesaplamak için kullanılır.

**Finality:** Bir bloğun geri alınamayacağının garantisi. GRANDPA, PezkuwiChain için kesinlik sağlar.

**FRAME (Framework for Runtime Aggregation of Modularized Entities):** Modüler palletler kullanarak blockchain runtimeları oluşturmak için bir Substrate çerçevesi.

**GRANDPA (GHOST-based Recursive ANcestor Deriving Prefix Agreement):** Validatörlerin blokların kesinliği konusunda anlaşmasına olanak tanıyan bir kesinlik aygıtı.

**Halving:** Token emisyon oranında periyodik azalma. PEZ, her 48 ayda bir sentetik yarılanmaya uğrar.

**HEZ:** PezkuwiChain'in yerel enflasyonist tokenı, stake etme, işlem ücretleri ve ağ güvenliği için kullanılır.

**Libp2p:** Node iletişimi için Substrate tarafından kullanılan modüler bir eşler arası ağ yığını.

**Nominator:** Güvenilen validatörleri desteklemek için HEZ tokenlerini stake eden bir ağ katılımcısı.

**NPoS (Nominated Proof-of-Stake):** Adayların validatörleri seçtiği Polkadot'un konsensüs mekanizması.

**Pallet:** Belirli işlevsellik sağlayan Substrate runtime'ındaki modüler bir bileşen.

**Parachain:** Paylaşımlı güvenlik ve birlikte çalışabilirlik için bir relay zincirine (Polkadot gibi) bağlanan bir blockchain.

**Parliamentary NFT:** Yönetişim hakları ve sahiplerine bonus ödülleri sağlayan 201 benzersiz NFT seti.

**PEZ:** PezkuwiChain'in sabit arzlı yönetişim tokenı (toplam 5 milyar), yönetişim ve ödüller için kullanılır.

**Planck:** HEZ tokeninin en küçük birimi (1 HEZ = 10^10 Planck).

**Runtime:** Blokların nasıl işlendiğini tanımlayan, blockchain'in durum geçiş fonksiyonu. Çatallanmasız yükseltmeler için WebAssembly olarak derlenir.

**Slashing:** Validatörlerin (ve adaylarının) yanlış davranışları için stake edilmiş tokenlerini kaybettikleri bir ceza mekanizması.

**Substrate:** PezkuwiChain'i oluşturmak için Parity Technologies tarafından geliştirilen blockchain çerçevesi.

**TNPoS (Trust-enhanced Nominated Proof-of-Stake):** PezkuwiChain'in, güven puanlarını validatör seçimi ve ödül dağıtımına entegre eden yenilikçi konsensüs mekanizması.

**Trust Score:** Kullanıcı davranışı, katkıları ve etkileşimleri temelinde `pallet-trust` tarafından hesaplanan bir itibar metriği.

**Unbonding Period:** Stake edilmemiş tokenlerin transfer edilebilir hale gelmeden önce kilitli kaldığı süre (28 gün).

**Validator:** Blok üreterek ve kesinlik için oylama yaparak konsensüse katılan bir node.

**Wasm (WebAssembly):** PezkuwiChain runtime'ı için kullanılan taşınabilir bir ikili talimat formatı, çatallanmasız yükseltmeleri etkinleştirir.

**welati:** PezkuwiChain için yönetişim paleti. Adı Kürtçe'de "vatandaş" anlamına gelir.

**perwerde:** Eğitim ve sertifikasyon paleti. Adı Kürtçe'de "eğitim" anlamına gelir.

**XCM (Cross-Consensus Messaging):** Polkadot ekosistemindeki farklı konsensüs sistemleri arasında iletişim için bir mesajlaşma formatı.

---

## 20. Appendix B: Developer Resources

### Başlarken
**Node Kurulumu:**
```bash
# Depoyu klonla
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk
# Node'u derle
cargo build --release
# Bir geliştirme node'u çalıştır
./target/release/pezkuwi-node --dev
```

**Testnet'e Bağlanma:**
```bash
# Beta Testnet'e bağlan
./target/release/pezkuwi-node \
--chain=beta \
--name="MyNode" \
--telemetry-url="wss://telemetry.pezkuwichain.io/submit 0"
```

### Pallet Entegrasyon Örnekleri
**`pallet-trust` Kullanımı:**
```rust
// Bir hesap için güven puanını al
let trust_score = pallet_trust::Pallet::<T>::get_trust_score(&account_id);
// Güven ilişkisi kur
pallet_trust::Pallet::<T>::add_trust(&truster, &trustee, trust_value)?;
```

**`pallet-pez-rewards` Kullanımı:**
```rust
// Dönem ödüllerini talep et
pallet_pez_rewards::Pallet::<T>::claim_rewards(origin, epoch_id)?;
// Parlamenter NFT sahipliğini kontrol et
let is_holder = pallet_pez_rewards::Pallet::<T>::is_parliamentary_nft_holder(&account);
```

**`identity-kyc` Kullanımı:**
```rust
// Kimliği kaydet
pallet_identity_kyc::Pallet::<T>::register_identity(
    origin,
    identity_data,
    kyc_level
)?;
// Kimliği doğrula
let is_verified = pallet_identity_kyc::Pallet::<T>::is_verified(&account_id);
```

### RPC Uç Noktaları
**Mainnet RPC:**
*   `wss://rpc.pezkuwichain.io`
*   `https://rpc.pezkuwichain.io`

**Testnet RPC:**
*   `wss://beta-rpc.pezkuwichain.io`
*   `https://beta-rpc.pezkuwichain.io`

### SDK'lar ve Kütüphaneler
**JavaScript/TypeScript:**
```bash
npm install @pezkuwichain/api
```
```typescript
import { ApiPromise, WsProvider } from '@pezkuwichain/api';
const provider = new WsProvider('wss://rpc.pezkuwichain.io');
const api = await ApiPromise.create({ provider });
// Güven puanını sorgula
const trustScore = await api.query.trust.trustScores(accountId);
```

**Python:**
```bash
pip install pezkuwichain-py
```
```python
from pezkuwichain import PezkuwiChain
chain = PezkuwiChain("wss://rpc.pezkuwichain.io")
trust_score = chain.query.trust.trust_scores(account_id)
```

### Test Etme
**Birim Testleri:**
```bash
cargo test --package pallet-trust
```
**Entegrasyon Testleri:**
```bash
cargo test --features runtime-benchmarks
```

### Benchmarking
```bash
./target/release/pezkuwi-node benchmark pallet \
--chain=dev \
--pallet=pallet_trust \
--extrinsic='*'
--steps=50 \
--repeat=20
```

### Dokümantasyon
*   **Runtime Dokümanları:** `cargo doc --open`
*   **Pallet Spesifikasyonları:** [https://docs.pezkuwichain.io/pallets](https://docs.pezkuwichain.app/pallets) 
*   **API Referansı:** [https://api.pezkuwichain.io/docs](https://api.pezkuwichain.app/docs)
*   **Eğitimler:** [https://developers.pezkuwichain.io/tutorials](https://developers.pezkuwichain.app/tutorials)

### Topluluk Desteği
*   **Geliştirici Forumu:** [https://forum.pezkuwichain.io](https://forum.pezkuwichain.io)
*   **Stack Overflow:** Etiket: pezkuwichain
*   **Discord #developers:** [https://discord.gg/pezkuwichain](https://discord.gg/pezkuwichain)
*   **Ofis Saatleri:** Haftalık geliştirici görüşmeleri (web sitesinde program).

**Doküman Sürümü:** 3.0
**Son Güncelleme:** November 2025
**Prepared by:** Kürdistan Teknoloji Bakanlığı & PezkuwiChain Katılımcıları
**Lisans:** Kurdistan Talent Institute License

Bu whitepaper, PezkuwiChain'in güncel vizyonunu ve teknik özelliklerini temsil etmektedir. Sürekli geliştirme aşamasında olan açık kaynaklı bir proje olarak, özellikler topluluk geri bildirimi, güvenlik denetimleri ve teknolojik gelişmeler temelinde evrimleşebilir. En güncel bilgiler için lütfen resmi GitHub deposuna ve dokümantasyona bakın.
