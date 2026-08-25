import { CloudAvailabilityStatus } from '@/lib/bindings'
import { useLocalization } from '@/hooks/use-localization'
import { CloudDownload, CloudCog } from 'lucide-react'
import { FC } from 'react'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

type Props = {
  status: CloudAvailabilityStatus | undefined
}

// FullyAvailable の場合（クラウド同期の対象外、または既にローカルに揃っている場合を含む）は
// 表示ノイズを避けるため何も表示しない。
export const AssetCardAvailabilityBadge: FC<Props> = ({ status }) => {
  const { t } = useLocalization()

  if (status === undefined || status === 'fullyAvailable') {
    return null
  }

  const Icon = status === 'notDownloaded' ? CloudDownload : CloudCog
  const labelKey =
    status === 'notDownloaded'
      ? 'asset-card:availability:not-downloaded'
      : 'asset-card:availability:partially-available'

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <div className="flex items-center justify-center size-8 rounded-md bg-muted text-muted-foreground shrink-0">
            <Icon className="size-4" />
          </div>
        </TooltipTrigger>
        <TooltipContent>
          <p>{t(labelKey)}</p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}
