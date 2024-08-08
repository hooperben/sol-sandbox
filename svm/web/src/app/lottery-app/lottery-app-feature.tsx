import { useWallet } from '@solana/wallet-adapter-react';
import { ExplorerLink } from '../cluster/cluster-ui';
import { WalletButton } from '../solana/solana-provider';
import { AppHero, ellipsify } from '../ui/ui-layout';
import { useLotteryAppProgram } from './lottery-app-data-access';
import { LotteryAppCreate, LotteryAppProgram } from './lottery-app-ui';

export default function LotteryAppFeature() {
  const { publicKey } = useWallet();
  const { programId } = useLotteryAppProgram();

  return publicKey ? (
    <div>
      <AppHero
        title="LotteryApp"
        subtitle={'Run the program by clicking the "Run program" button.'}
      >
        <p className="mb-6">
          <ExplorerLink
            path={`account/${programId}`}
            label={ellipsify(programId.toString())}
          />
        </p>
        <LotteryAppCreate />
      </AppHero>
      <LotteryAppProgram />
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton className="btn btn-primary" />
        </div>
      </div>
    </div>
  );
}
