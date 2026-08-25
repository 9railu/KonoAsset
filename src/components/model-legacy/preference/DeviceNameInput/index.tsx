import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'

import { FC, useState, useEffect } from 'react'
import { useLocalization } from '@/hooks/use-localization'

type Props = {
  deviceName: string
  setDeviceName: (deviceName: string) => void
}

export const DeviceNameInput: FC<Props> = ({ deviceName, setDeviceName }) => {
  const { t } = useLocalization()
  const [value, setValue] = useState(deviceName)

  useEffect(() => {
    setValue(deviceName)
  }, [deviceName])

  return (
    <div className="flex flex-row items-center w-full gap-8">
      <div className="space-y-2 shrink min-w-0 overflow-hidden">
        <Label className="text-lg">
          {t('preference:settings:device-name-input')}
        </Label>
        <p className="text-muted-foreground text-sm wrap-break-word">
          {t('preference:settings:device-name-input:explanation-text')}
        </p>
      </div>
      <Input
        className="ml-auto shrink-0 w-64"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        onBlur={() => {
          const trimmed = value.trim()
          if (trimmed.length > 0 && trimmed !== deviceName) {
            setDeviceName(trimmed)
          } else {
            setValue(deviceName)
          }
        }}
      />
    </div>
  )
}
