export type Alert = {
  type: 'error' | 'warning' | 'success'
  title: string
  message: string
}