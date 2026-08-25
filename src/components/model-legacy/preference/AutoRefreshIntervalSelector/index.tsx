import { Label } from '@/components/ui/label'
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from '@/components/ui/select'
import { FC } from 'react'
import { useLocalization } from '@/hooks/use-localization'

type Props = {
  intervalSeconds: number
  setIntervalSeconds: (intervalSeconds: number) => void
}

const options: { seconds: number; labelKey: string }[] = [
  { seconds: 0, labelKey: 'preference:settings:auto-refresh-interval:off' },
  {
    seconds: 30,
    labelKey: 'preference:settings:auto-refresh-interval:30-seconds',
  },
  {
    seconds: 60,
    labelKey: 'preference:settings:auto-refresh-interval:1-minute',
  },
  {
    seconds: 300,
    labelKey: 'preference:settings:auto-refresh-interval:5-minutes',
  },
  {
    seconds: 900,
    labelKey: 'preference:settings:auto-refresh-interval:15-minutes',
  },
]

export const AutoRefreshIntervalSelector: FC<Props> = ({
  intervalSeconds,
  setIntervalSeconds,
}) => {
  const { t } = useLocalization()

  return (
    <div className="flex flex-row items-center">
      <div className="space-y-2 mr-2">
        <Label className="text-xl">
          {t('preference:settings:auto-refresh-interval')}
        </Label>
        <p className="text-muted-foreground text-sm">
          {t('preference:settings:auto-refresh-interval:explanation-text')}
        </p>
      </div>
      <Select
        value={String(intervalSeconds)}
        onValueChange={(value) => setIntervalSeconds(Number(value))}
      >
        <SelectTrigger className="ml-auto w-[220px]">
          <SelectValue placeholder={t('general:select:placeholder')} />
        </SelectTrigger>
        <SelectContent>
          {options.map(({ seconds, labelKey }) => (
            <SelectItem key={seconds} value={String(seconds)}>
              <span className="break-all">{t(labelKey)}</span>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  )
}
