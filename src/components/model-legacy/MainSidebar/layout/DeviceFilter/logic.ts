import { Option } from '@/components/ui/multi-select'
import { commands } from '@/lib/bindings'

export type DeviceOption = Option & { id: string }

export const fetchRegisteredDevices = async (): Promise<DeviceOption[]> => {
  const result = await commands.getRegisteredDeviceNames()

  if (result.status === 'error') {
    console.error(result.error)
    return []
  }

  return result.data.map((device) => ({
    id: device.id,
    value: device.name,
  }))
}
