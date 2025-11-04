#!/usr/bin/env python3
"""
Arbitrage Bot for PezkuwiChain DEX
Monitors wHEZ/PEZ pool and executes arbitrage when price deviates from reference
"""

import json
import time
from datetime import datetime
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# Colors
class Colors:
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    RED = '\033[0;31m'
    CYAN = '\033[0;36m'
    BLUE = '\033[0;34m'
    MAGENTA = '\033[0;35m'
    NC = '\033[0m'

def print_colored(message, color):
    print(f"{color}{message}{Colors.NC}")

# Constants
ONE_TOKEN = 10**12

# Configuration
CONFIG = {
    'ws_url': 'ws://127.0.0.1:9944',
    'reference_price': 5.0,  # 1 HEZ = 5 PEZ (reference/target price)
    'min_profit_percent': 2.0,  # Minimum profit percentage to execute arbitrage
    'max_swap_amount_hez': 5000,  # Maximum HEZ to swap in single transaction
    'check_interval': 30,  # Seconds between price checks
    'slippage_tolerance': 0.05,  # 5% slippage tolerance
}

class ArbitrageBot:
    def __init__(self, keypair_seed, config):
        self.config = config
        self.keypair = Keypair.create_from_mnemonic(keypair_seed)
        self.substrate = None
        self.total_profit = 0
        self.trades_executed = 0

    def connect(self):
        """Connect to substrate node"""
        print_colored(f"🔗 Connecting to {self.config['ws_url']}...", Colors.CYAN)
        try:
            self.substrate = SubstrateInterface(
                url=self.config['ws_url'],
                ss58_format=42,
                type_registry_preset='substrate-node-template'
            )
            print_colored("✅ Connected successfully!", Colors.GREEN)
            print_colored(f"🔑 Bot address: {self.keypair.ss58_address}", Colors.YELLOW)
            return True
        except Exception as e:
            print_colored(f"❌ Connection failed: {e}", Colors.RED)
            return False

    def get_pool_price(self):
        """Get current pool price and reserves"""
        try:
            # Query pool
            pool_id = (0, 1)  # wHEZ (0) / PEZ (1) - use tuple, not list
            pool_info = self.substrate.query('AssetConversion', 'Pools', [pool_id])

            if not pool_info.value:
                print_colored("❌ Pool not found", Colors.RED)
                return None

            # Get pool account
            pool_account_info = self.substrate.query('AssetConversion', 'PoolAccountIds', [pool_id])
            if not pool_account_info.value:
                return None

            pool_account = pool_account_info.value

            # Get reserves
            whez_balance = self.substrate.query('Assets', 'Account', [0, pool_account])
            pez_balance = self.substrate.query('Assets', 'Account', [1, pool_account])

            if not whez_balance.value or not pez_balance.value:
                return None

            reserve_whez = whez_balance.value['balance'] / ONE_TOKEN
            reserve_pez = pez_balance.value['balance'] / ONE_TOKEN

            # Calculate price (1 HEZ = X PEZ)
            price = reserve_pez / reserve_whez

            return {
                'price': price,
                'reserve_whez': reserve_whez,
                'reserve_pez': reserve_pez,
                'pool_account': pool_account
            }

        except Exception as e:
            print_colored(f"❌ Error getting pool price: {e}", Colors.RED)
            return None

    def calculate_arbitrage(self, pool_data):
        """Calculate arbitrage opportunity"""
        pool_price = pool_data['price']
        reference_price = self.config['reference_price']

        # Price deviation percentage
        deviation = ((pool_price - reference_price) / reference_price) * 100

        print_colored(f"📊 Pool Price: 1 HEZ = {pool_price:.4f} PEZ", Colors.CYAN)
        print_colored(f"📊 Reference: 1 HEZ = {reference_price:.4f} PEZ", Colors.CYAN)
        print_colored(f"📊 Deviation: {deviation:+.2f}%", Colors.YELLOW if abs(deviation) > self.config['min_profit_percent'] else Colors.GREEN)

        if abs(deviation) < self.config['min_profit_percent']:
            return None

        # Determine arbitrage direction
        if pool_price > reference_price:
            # HEZ is overpriced in pool → sell HEZ for PEZ
            direction = 'HEZ_TO_PEZ'
            message = f"💰 HEZ overpriced by {deviation:.2f}% → Sell HEZ for PEZ"
        else:
            # HEZ is underpriced in pool → buy HEZ with PEZ
            direction = 'PEZ_TO_HEZ'
            message = f"💰 HEZ underpriced by {abs(deviation):.2f}% → Buy HEZ with PEZ"

        print_colored(message, Colors.GREEN)

        # Calculate optimal swap amount (simplified)
        # In real scenario, this would be more sophisticated
        optimal_amount = min(
            self.config['max_swap_amount_hez'],
            pool_data['reserve_whez'] * 0.05  # Max 5% of pool
        )

        return {
            'direction': direction,
            'amount': optimal_amount,
            'expected_profit_percent': abs(deviation),
            'pool_data': pool_data
        }

    def execute_arbitrage(self, arb_opportunity):
        """Execute arbitrage swap"""
        direction = arb_opportunity['direction']
        amount = arb_opportunity['amount']

        print_colored(f"\n🔄 Executing arbitrage: {direction}", Colors.MAGENTA)
        print_colored(f"   Amount: {amount:.2f} tokens", Colors.YELLOW)

        try:
            if direction == 'HEZ_TO_PEZ':
                # Swap HEZ → wHEZ → PEZ
                return self._swap_hez_to_pez(amount)
            else:
                # Swap PEZ → wHEZ → HEZ
                return self._swap_pez_to_hez(amount)

        except Exception as e:
            print_colored(f"❌ Arbitrage execution failed: {e}", Colors.RED)
            return False

    def _swap_hez_to_pez(self, hez_amount):
        """Swap HEZ to PEZ (wrap + swap)"""
        amount_in = int(hez_amount * ONE_TOKEN)

        # Step 1: Wrap HEZ to wHEZ
        print_colored("   Step 1: Wrapping HEZ to wHEZ...", Colors.CYAN)
        wrap_call = self.substrate.compose_call(
            call_module='TokenWrapper',
            call_function='wrap',
            call_params={'amount': amount_in}
        )

        # Step 2: Swap wHEZ to PEZ
        print_colored("   Step 2: Swapping wHEZ to PEZ...", Colors.CYAN)
        swap_path = [0, 1]  # wHEZ → PEZ

        # Calculate minimum output (with slippage)
        # Simplified: use 95% of expected output
        min_amount_out = int(amount_in * 0.95 * 4.85)  # ~97% of 5 (after 3% fee)

        swap_call = self.substrate.compose_call(
            call_module='AssetConversion',
            call_function='swap_exact_tokens_for_tokens',
            call_params={
                'path': swap_path,
                'amount_in': amount_in,
                'amount_out_min': min_amount_out,
                'send_to': self.keypair.ss58_address,
                'keep_alive': True
            }
        )

        # Batch the calls
        batch_call = self.substrate.compose_call(
            call_module='Utility',
            call_function='batch_all',
            call_params={'calls': [wrap_call, swap_call]}
        )

        # Submit transaction
        extrinsic = self.substrate.create_signed_extrinsic(call=batch_call, keypair=self.keypair)
        receipt = self.substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        if receipt.is_success:
            print_colored(f"✅ Swap successful! Block: {receipt.block_hash}", Colors.GREEN)
            self.trades_executed += 1
            return True
        else:
            print_colored(f"❌ Swap failed: {receipt.error_message}", Colors.RED)
            return False

    def _swap_pez_to_hez(self, pez_amount):
        """Swap PEZ to HEZ (swap + unwrap)"""
        amount_in = int(pez_amount * ONE_TOKEN)

        # Step 1: Swap PEZ to wHEZ
        print_colored("   Step 1: Swapping PEZ to wHEZ...", Colors.CYAN)
        swap_path = [1, 0]  # PEZ → wHEZ

        # Calculate minimum output
        min_amount_out = int(amount_in * 0.95 * 0.19)  # ~97% of 0.2 (after 3% fee)

        swap_call = self.substrate.compose_call(
            call_module='AssetConversion',
            call_function='swap_exact_tokens_for_tokens',
            call_params={
                'path': swap_path,
                'amount_in': amount_in,
                'amount_out_min': min_amount_out,
                'send_to': self.keypair.ss58_address,
                'keep_alive': True
            }
        )

        # Step 2: Unwrap wHEZ to HEZ
        print_colored("   Step 2: Unwrapping wHEZ to HEZ...", Colors.CYAN)
        unwrap_call = self.substrate.compose_call(
            call_module='TokenWrapper',
            call_function='unwrap',
            call_params={'amount': min_amount_out}
        )

        # Batch the calls
        batch_call = self.substrate.compose_call(
            call_module='Utility',
            call_function='batch_all',
            call_params={'calls': [swap_call, unwrap_call]}
        )

        # Submit transaction
        extrinsic = self.substrate.create_signed_extrinsic(call=batch_call, keypair=self.keypair)
        receipt = self.substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        if receipt.is_success:
            print_colored(f"✅ Swap successful! Block: {receipt.block_hash}", Colors.GREEN)
            self.trades_executed += 1
            return True
        else:
            print_colored(f"❌ Swap failed: {receipt.error_message}", Colors.RED)
            return False

    def run(self):
        """Main bot loop"""
        print_colored("\n" + "="*70, Colors.CYAN)
        print_colored("   🤖 Arbitrage Bot Started", Colors.CYAN)
        print_colored("="*70, Colors.CYAN)
        print_colored(f"\n⚙️  Configuration:", Colors.BLUE)
        print_colored(f"   Reference Price: 1 HEZ = {self.config['reference_price']} PEZ", Colors.YELLOW)
        print_colored(f"   Min Profit: {self.config['min_profit_percent']}%", Colors.YELLOW)
        print_colored(f"   Max Swap: {self.config['max_swap_amount_hez']} HEZ", Colors.YELLOW)
        print_colored(f"   Check Interval: {self.config['check_interval']}s\n", Colors.YELLOW)

        iteration = 0

        try:
            while True:
                iteration += 1
                timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

                print_colored(f"\n{'─'*70}", Colors.CYAN)
                print_colored(f"🔍 Check #{iteration} - {timestamp}", Colors.BLUE)
                print_colored(f"{'─'*70}", Colors.CYAN)

                # Get pool price
                pool_data = self.get_pool_price()

                if not pool_data:
                    print_colored("⚠️  Could not fetch pool data, retrying...", Colors.YELLOW)
                    time.sleep(self.config['check_interval'])
                    continue

                # Calculate arbitrage opportunity
                arb_opportunity = self.calculate_arbitrage(pool_data)

                if arb_opportunity:
                    print_colored(f"\n💡 Arbitrage opportunity detected!", Colors.GREEN)
                    print_colored(f"   Expected profit: {arb_opportunity['expected_profit_percent']:.2f}%", Colors.GREEN)

                    # Execute arbitrage
                    success = self.execute_arbitrage(arb_opportunity)

                    if success:
                        print_colored(f"\n✨ Arbitrage executed successfully!", Colors.GREEN)
                        print_colored(f"   Total trades: {self.trades_executed}", Colors.YELLOW)
                    else:
                        print_colored(f"\n⚠️  Arbitrage execution failed", Colors.YELLOW)
                else:
                    print_colored("\n😴 No arbitrage opportunity (price within acceptable range)", Colors.GREEN)

                print_colored(f"\n💤 Sleeping for {self.config['check_interval']} seconds...", Colors.CYAN)
                time.sleep(self.config['check_interval'])

        except KeyboardInterrupt:
            print_colored("\n\n⚠️  Bot stopped by user", Colors.YELLOW)
            print_colored(f"\n📊 Session Summary:", Colors.BLUE)
            print_colored(f"   Total trades executed: {self.trades_executed}", Colors.YELLOW)
            print_colored(f"   Total iterations: {iteration}", Colors.YELLOW)
        except Exception as e:
            print_colored(f"\n❌ Fatal error: {e}", Colors.RED)
            import traceback
            traceback.print_exc()
        finally:
            if self.substrate:
                self.substrate.close()
                print_colored("\n👋 Connection closed", Colors.CYAN)

def main():
    # Bot configuration
    # IMPORTANT: In production, use secure key management!
    # This is just for testing with the Founder account
    FOUNDER_SEED = "skill dose toward always latin fish film cabbage praise blouse kingdom depth"

    # Create and run bot
    bot = ArbitrageBot(FOUNDER_SEED, CONFIG)

    if bot.connect():
        bot.run()
    else:
        print_colored("❌ Failed to start bot", Colors.RED)
        return 1

    return 0

if __name__ == "__main__":
    exit(main())
