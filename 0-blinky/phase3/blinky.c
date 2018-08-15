#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_FSEL0 = (volatile unsigned *)(GPIO_BASE);
volatile unsigned *GPIO_SET0 = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0 = (volatile unsigned *)(GPIO_BASE + 0x28);

static unsigned int const GPIO_MASK = 0b111;
static unsigned int const GPIO_OUT = 0b001;
static unsigned int const GPIO_IN = 0b000;

static void spin_sleep_us(unsigned int us)
{
  for (unsigned int i = 0; i < us * 6; i++)
  {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms)
{
  spin_sleep_us(ms * 1000);
}

static void fsel_set(unsigned int pin_num, unsigned int set)
{
  if (pin_num > 53)
    return;
  // each 32-bit wide space fits 10 FSEL, and our mask is size 3
  unsigned int shift = (pin_num % 10) * 3;
  // to get which FSEL(n), integer div by 10 the pin number, as they are in chunks of 10
  unsigned int offset = pin_num / 10;
  *(GPIO_FSEL0 + offset) = (*(GPIO_FSEL0 + offset) & ~(GPIO_MASK << shift)) | (set << shift);
  return;
}

static void fsel_in(unsigned int pin_num)
{
  fsel_set(pin_num, GPIO_IN);
}

static void fsel_out(unsigned int pin_num)
{
  fsel_set(pin_num, GPIO_OUT);
}

// There are 2 registers each for SET & CLR. We figure out the offset by num `mod` address_width
static void pin_set(unsigned int pin_num)
{
  if (pin_num > 53)
    return;
  unsigned int set_num = pin_num % 32;
  *(GPIO_SET0 + set_num) = 1 << pin_num; // same as GPIO_SET0[set_num]
  return;
}

static void pin_clear(unsigned int pin_num)
{
  if (pin_num > 53)
    return;
  unsigned int set_num = pin_num % 32;
  *(GPIO_CLR0 + set_num) = 1 << pin_num;
  return;
}
// Q: Now, read the documentation for GPFSELn register on pages 91 and 92.
// We write to this register to set up a pin as an output or input.
// Which value to which field in register GPFSEL1 must be written so that GPIO pin 16
// is set as an output?
// A: 0b001 according to the docs
// I think this would work *GPIO_FSEL1 |= 0b001 << 18; but I want to figure out the general case
int main(void)
{
  // STEP 1: Set GPIO Pin 16 as output.
  fsel_out(16); // *GPIO_FSEL1 |= 0b001 << 18;
  // STEP 2: Continuously set and clear GPIO 16.
  while (1)
  {
    pin_set(16);
    spin_sleep_ms(1000);
    pin_clear(16);
    spin_sleep_ms(1000);
  }
}
