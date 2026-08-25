import { Card } from '@/components/ui/card'
import { FC } from 'react'
import { Button } from '@/components/ui/button'
import { useLocalization } from '@/hooks/use-localization'
import { StatusBarOptionPopover } from '../../StatusBarOptionPopover'
import { cn } from '@/lib/utils'
import { RefreshCw } from 'lucide-react'

type Props = {
  totalAssetCount: number
  filterAppliedAssetCount?: number
  clearFilters: () => void
  onRefresh: () => void
  isRefreshing: boolean
}

export const InternalStatusBar: FC<Props> = ({
  totalAssetCount,
  filterAppliedAssetCount,
  clearFilters,
  onRefresh,
  isRefreshing,
}) => {
  const { t } = useLocalization()

  const displayAmount = filterAppliedAssetCount ?? totalAssetCount

  return (
    <div className="w-full flex flex-row p-4">
      <div className="w-full shrink">
        <Card className="h-10 flex flex-row items-center p-0 gap-0">
          <div className={cn('w-0 lg:w-28')} />
          <p className="my-auto ml-4 lg:mx-auto text-muted-foreground line-clamp-1">
            <span>
              {t('mainnavbar:total-asset-count:foretext')}
              {totalAssetCount}
              {t('mainnavbar:total-asset-count:posttext')}
            </span>
            <span className="text-card-foreground font-bold px-1 rounded-lg">
              {displayAmount}
              {t('general:count')}
            </span>
            <span>{t('mainnavbar:showing')}</span>
          </p>
          <div className="w-28 ml-auto mr-1 flex justify-end">
            <Button
              className={cn(
                'h-8',
                totalAssetCount === displayAmount && 'hidden',
              )}
              variant="secondary"
              onClick={clearFilters}
            >
              {t('general:clear-filter')}
            </Button>
          </div>
        </Card>
      </div>
      <div className="ml-2 flex items-center shrink-0 gap-1">
        <Button
          className="h-8 w-8"
          variant="secondary"
          size="icon"
          onClick={onRefresh}
          disabled={isRefreshing}
          title={t('mainnavbar:refresh')}
        >
          <RefreshCw className={cn('h-4 w-4', isRefreshing && 'animate-spin')} />
        </Button>
        <StatusBarOptionPopover />
      </div>
    </div>
  )
}
