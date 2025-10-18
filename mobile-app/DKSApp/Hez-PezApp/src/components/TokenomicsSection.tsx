import React, { useState, useEffect } from 'react';
import { PieChart, Clock, TrendingDown, Coins } from 'lucide-react';

const TokenomicsSection: React.FC = () => {
  const [monthsPassed, setMonthsPassed] = useState(0);
  const [currentRelease, setCurrentRelease] = useState(0);
  
  // Calculate halving period
  const halvingPeriod = Math.floor(monthsPassed / 48);
  const monthsUntilNextHalving = 48 - (monthsPassed % 48);
  
  // Calculate monthly release amount
  useEffect(() => {
    const baseAmount = 74218750; // Initial monthly release in PEZ
    const release = baseAmount / Math.pow(2, halvingPeriod);
    setCurrentRelease(release);
  }, [monthsPassed, halvingPeriod]);

  const distribution = [
    { name: 'Treasury', percentage: 96.25, amount: 4812500000, color: 'from-purple-500 to-purple-600' },
    { name: 'Presale', percentage: 1.875, amount: 93750000, color: 'from-cyan-500 to-cyan-600' },
    { name: 'Founder', percentage: 1.875, amount: 93750000, color: 'from-teal-500 to-teal-600' }
  ];

  return (
    <section id="tokenomics" className="py-20 bg-gray-900/50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-12">
          <h2 className="text-4xl font-bold mb-4 bg-gradient-to-r from-purple-400 to-cyan-400 bg-clip-text text-transparent">
            PEZ Tokenomics
          </h2>
          <p className="text-gray-400 text-lg max-w-2xl mx-auto">
            5 Billion total supply with programmatic distribution and synthetic halving
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Distribution Chart */}
          <div className="bg-gray-950/50 backdrop-blur-sm rounded-xl border border-gray-800 p-6">
            <div className="flex items-center mb-6">
              <PieChart className="w-6 h-6 text-purple-400 mr-3" />
              <h3 className="text-xl font-semibold text-white">Genesis Distribution</h3>
            </div>
            
            <div className="flex justify-center mb-6">
              <img 
                src="https://d64gsuwffb70l.cloudfront.net/68ec477a0a2fa844d6f9df15_1760315334095_faa6af97.webp"
                alt="Token Distribution"
                className="w-48 h-48 rounded-lg"
              />
            </div>

            <div className="space-y-3">
              {distribution.map((item) => (
                <div key={item.name} className="flex items-center justify-between p-3 bg-gray-900/50 rounded-lg">
                  <div className="flex items-center">
                    <div className={`w-3 h-3 rounded-full bg-gradient-to-r ${item.color} mr-3`}></div>
                    <span className="text-gray-300">{item.name}</span>
                  </div>
                  <div className="text-right">
                    <div className="text-white font-semibold">{item.percentage}%</div>
                    <div className="text-gray-500 text-sm">{item.amount.toLocaleString()} PEZ</div>
                  </div>
                </div>
              ))}
            </div>

            <div className="mt-6 p-4 bg-kurdish-yellow/20 rounded-lg border border-kurdish-yellow/30">
              <div className="flex items-center justify-between">
                <span className="text-purple-400">Total Supply</span>
                <span className="text-white font-bold">5,000,000,000 PEZ</span>
              </div>
            </div>
          </div>

          {/* Halving Mechanism */}
          <div className="bg-gray-950/50 backdrop-blur-sm rounded-xl border border-gray-800 p-6">
            <div className="flex items-center mb-6">
              <Clock className="w-6 h-6 text-cyan-400 mr-3" />
              <h3 className="text-xl font-semibold text-white">Synthetic Halving</h3>
            </div>

            <div className="space-y-4">
              <div className="p-4 bg-gray-900/50 rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-gray-400">Current Period</span>
                  <span className="text-white font-semibold">Period {halvingPeriod + 1}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">Next Halving</span>
                  <span className="text-cyan-400">{monthsUntilNextHalving} months</span>
                </div>
              </div>

              <div className="p-4 bg-gradient-to-r from-purple-900/20 to-cyan-900/20 rounded-lg border border-purple-500/30">
                <div className="flex items-center mb-3">
                  <TrendingDown className="w-5 h-5 text-purple-400 mr-2" />
                  <span className="text-gray-300">Monthly Release Schedule</span>
                </div>
                <div className="text-2xl font-bold text-white mb-1">
                  {currentRelease.toLocaleString()} PEZ
                </div>
                <div className="text-sm text-gray-400">Per month in current period</div>
              </div>

              {/* Halving Timeline */}
              <div className="mt-6">
                <h4 className="text-gray-300 mb-3">Halving Timeline</h4>
                <div className="space-y-2">
                  {[0, 1, 2, 3].map((period) => (
                    <div key={period} className="flex items-center">
                      <div className={`w-2 h-2 rounded-full ${period <= halvingPeriod ? 'bg-cyan-400' : 'bg-gray-600'} mr-3`}></div>
                      <span className={`text-sm ${period <= halvingPeriod ? 'text-gray-300' : 'text-gray-500'}`}>
                        Period {period + 1}: {(74218750 / Math.pow(2, period)).toLocaleString()} PEZ/month
                      </span>
                    </div>
                  ))}
                </div>
              </div>

              {/* Interactive Slider */}
              <div className="mt-6">
                <label className="text-gray-300 text-sm">Simulate Timeline (Months)</label>
                <input
                  type="range"
                  min="0"
                  max="192"
                  value={monthsPassed}
                  onChange={(e) => setMonthsPassed(parseInt(e.target.value))}
                  className="w-full mt-2"
                />
                <div className="flex justify-between text-xs text-gray-500 mt-1">
                  <span>0</span>
                  <span>48</span>
                  <span>96</span>
                  <span>144</span>
                  <span>192</span>
                </div>
              </div>

              <div className="flex items-center justify-between p-3 bg-cyan-900/20 rounded-lg">
                <div className="flex items-center">
                  <Coins className="w-4 h-4 text-cyan-400 mr-2" />
                  <span className="text-gray-300 text-sm">Total Released</span>
                </div>
                <span className="text-cyan-400 font-semibold">
                  {Math.min(monthsPassed * currentRelease, 4812500000).toLocaleString()} PEZ
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default TokenomicsSection;