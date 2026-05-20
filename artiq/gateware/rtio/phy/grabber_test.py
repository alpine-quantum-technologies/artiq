# from artiq.experiment import *


# class GrabberRoiExperiment(EnvExperiment):
#     def build(self):
#         # 1. Request core and the grabber hardware device
#         self.setattr_device("core")
#         self.setattr_device("grabber0")

#         # Assume your camera trigger is connected to a standard TTL output channel
#         self.setattr_device("ttl4")

#     @kernel
#     def run(self):
#         self.core.reset()

#         self.grabber0.setup_roi(0, 100, 200, 150, 250)
#         delay(10 * us)  # Small RTIO slack cushion between register writes

#         self.grabber0.setup_roi(1, 400, 500, 300, 400)
#         delay(10 * us)

#         self.grabber0.gate_roi(0xFFFFFFFF)
#         delay(10 * us)

#         self.ttl4.pulse(1 * ms)

#         delay(50 * ms)

#         self.grabber0.gate_roi(0x0)

#         delay(10*us)

#         roi_0_data = [0 for _ in range(2)]
#         roi_1_data = [0 for _ in range(2)]

#         n = [0]*2
#         self.grabber0.input_mu(n)

#         print(n)

from artiq.experiment import *
from numpy import int32


class GrabberLoopExperiment(EnvExperiment):
    def build(self):
        self.setattr_device("core")
        self.setattr_device("grabber0")
        self.setattr_device("ttl4")

        # Define how many ROIs you want to loop over (e.g., 4 ROIs)
        # Note: Ensure this doesn't exceed your gateware's compiled max ROI engine count!
        self.num_rois = 32

    @kernel
    def run(self):
        self.core.reset()

        for i in range(self.num_rois):
            offset = i * 5
            self.grabber0.setup_roi(
                int32(i), 
                100 + offset, 
                200 + offset, 
                150 + offset, 
                250 + offset
            )
            delay(10 * us)

        mask = (1 << self.num_rois) - 1
        self.grabber0.gate_roi(mask)
        delay(10 * us)

        self.ttl4.pulse(1 * ms)
        delay(50 * ms)

        self.grabber0.gate_roi(0x0)
        delay(10 * us)

        n = [0] * self.num_rois
        
        self.grabber0.input_mu(n)

        print("Acquired ROI integrated values:")
        for i in range(self.num_rois):
            print("ROI ", i, " -> ", n[i])