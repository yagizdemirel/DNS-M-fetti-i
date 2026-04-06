document.addEventListener('DOMContentLoaded', () => {
    const tbody = document.getElementById('reports-body');
    const totalRequestsEl = document.getElementById('total-requests');
    const lastActivityEl = document.getElementById('last-activity');
    const refreshBtn = document.getElementById('refresh-btn');

    let isFetching = false;
    let oldDataLength = 0;

    const fetchReports = async () => {
        if (isFetching) return;
        isFetching = true;

        try {
            // API'den veri çek (Aynı origin üzerinden çalıştığı için relative path kullanıyoruz)
            const response = await fetch('/api/reports');
            if (!response.ok) throw new Error('API Hatası');
            
            const data = await response.json();
            
            // Eğer yeni veri yoksa DOM'u gereksiz güncelleme
            if (data.length === 0) {
                tbody.innerHTML = '<tr><td colspan="5" style="text-align: center; color: #94a3b8;">Henüz yakalanan bir istek yok...</td></tr>';
                isFetching = false;
                return;
            }

            // Arayüz İstatistiklerini Güncelle
            renderStats(data);

            // Tabloyu yeniden oluştur
            renderTable(data);
            
            oldDataLength = data.length;
        } catch (error) {
            console.error("Raporlar çekilemedi:", error);
        } finally {
            isFetching = false;
        }
    };

    const renderStats = (data) => {
        // En yüksek ID toplam istek sayısını verir (ya da liste uzunluğu)
        const total = data.length > 0 ? data[0].id : 0; 
        
        // Animasyonlu sayı artışı (Basit sürüm)
        totalRequestsEl.textContent = total;
        
        if (data.length > 0) {
            lastActivityEl.textContent = data[0].timestamp;
        }
    };

    const renderTable = (data) => {
        // Performans için sadece ilk 50 sonucu arayüzde gösterelim
        const displayData = data.slice(0, 50);
        
        tbody.innerHTML = displayData.map(log => {
            const isSuccess = log.status.includes('Çözüldü');
            const badgeClass = isSuccess ? 'status-success' : 'status-error';
            
            // Satır HTML'i
            return `
                <tr>
                    <td>#${log.id}</td>
                    <td style="color: #94a3b8;">${log.timestamp}</td>
                    <td class="domain-label">${log.domain}</td>
                    <td><span style="background: rgba(255,255,255,0.1); padding: 2px 8px; border-radius: 4px; font-size: 0.8rem;">${log.record_type}</span></td>
                    <td><span class="status-badge ${badgeClass}">${log.status}</span></td>
                </tr>
            `;
        }).join('');
    };

    // İlk yükleme
    fetchReports();

    // Manuel Yenileme
    refreshBtn.addEventListener('click', () => {
        refreshBtn.innerText = "Yenileniyor...";
        fetchReports().then(() => {
            setTimeout(() => refreshBtn.innerText = "Yenile ⟳", 500);
        });
    });

    // Otomatik Yenileme (Her 2 saniyede bir kontrol et)
    setInterval(fetchReports, 2000);
});
